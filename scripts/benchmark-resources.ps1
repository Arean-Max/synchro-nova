Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public class Win32 {
    [DllImport("user32.dll")]
    public static extern uint GetGuiResources(IntPtr hProcess, uint uiFlags);

    [DllImport("user32.dll")]
    public static extern bool ShowWindow(IntPtr hWnd, int nCmdShow);

    [DllImport("user32.dll")]
    public static extern bool SetForegroundWindow(IntPtr hWnd);

    [DllImport("psapi.dll")]
    public static extern int EmptyWorkingSet(IntPtr hwProc);
}
"@

Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing

$settingsPath = "$env:APPDATA\app.synchro.performance\settings.json"
$appPath = "$PSScriptRoot\..\release\synchro.exe"
$appDir = "$PSScriptRoot\..\release"

function Get-SynchroTree {
    param([int]$ParentPid)
    
    $allProcesses = Get-CimInstance Win32_Process
    $matchedPids = [System.Collections.Generic.HashSet[int]]::new()
    $matchedPids.Add($ParentPid) | Out-Null

    $changed = $true
    while ($changed) {
        $changed = $false
        foreach ($proc in $allProcesses) {
            if ($matchedPids.Contains($proc.ParentProcessId) -and -not $matchedPids.Contains($proc.ProcessId)) {
                $matchedPids.Add($proc.ProcessId) | Out-Null
                $changed = $true
            }
        }
    }

    $procs = @()
    foreach ($pidNum in $matchedPids) {
        $p = Get-Process -Id $pidNum -ErrorAction SilentlyContinue
        if ($p) {
            $procs += $p
        }
    }
    return $procs
}

function Measure-ResourceMetrics {
    param(
        [int]$ParentPid,
        [int]$DurationSec = 8
    )

    $samples = @()
    $cpuCount = [Environment]::ProcessorCount
    $intervalMs = 500
    $sampleCount = [math]::Max(1, [int]($DurationSec * 1000 / $intervalMs))

    $procs = Get-SynchroTree -ParentPid $ParentPid
    $prevTimes = @{}
    foreach ($p in $procs) {
        try {
            $prevTimes[$p.Id] = $p.TotalProcessorTime.TotalMilliseconds
        } catch {}
    }
    $prevStamp = [System.Diagnostics.Stopwatch]::GetTimestamp()

    for ($i = 0; $i -lt $sampleCount; $i++) {
        Start-Sleep -Milliseconds $intervalMs
        $curStamp = [System.Diagnostics.Stopwatch]::GetTimestamp()
        $elapsedMs = (($curStamp - $prevStamp) / [System.Diagnostics.Stopwatch]::Frequency) * 1000.0
        $prevStamp = $curStamp

        $procs = Get-SynchroTree -ParentPid $ParentPid
        if ($procs.Count -eq 0) { break }

        $totalCpuMs = 0.0
        $totalWs = 0L
        $totalPrivate = 0L
        $totalPaged = 0L
        $totalNonpaged = 0L
        $totalHandles = 0
        $totalThreads = 0
        $totalGdi = 0
        $totalUser = 0
        $totalReadBytes = 0L
        $totalWriteBytes = 0L

        $mainWs = 0L
        $mainPrivate = 0L
        $webviewWs = 0L
        $webviewPrivate = 0L

        foreach ($p in $procs) {
            try {
                $curCpu = $p.TotalProcessorTime.TotalMilliseconds
                $prevCpu = if ($prevTimes.ContainsKey($p.Id)) { $prevTimes[$p.Id] } else { $curCpu }
                $deltaCpu = [math]::Max(0.0, ($curCpu - $prevCpu))
                $totalCpuMs += $deltaCpu
                $prevTimes[$p.Id] = $curCpu

                $totalWs += $p.WorkingSet64
                $totalPrivate += $p.PrivateMemorySize64
                $totalPaged += $p.PagedMemorySize64
                $totalNonpaged += $p.NonpagedSystemMemorySize64
                $totalHandles += $p.HandleCount
                $totalThreads += $p.Threads.Count

                if ($p.Id -eq $ParentPid) {
                    $mainWs += $p.WorkingSet64
                    $mainPrivate += $p.PrivateMemorySize64
                } else {
                    $webviewWs += $p.WorkingSet64
                    $webviewPrivate += $p.PrivateMemorySize64
                }

                $gdi = [Win32]::GetGuiResources($p.Handle, 0)
                $usr = [Win32]::GetGuiResources($p.Handle, 1)
                $totalGdi += $gdi
                $totalUser += $usr

                $totalReadBytes += $p.ReadTransferCount
                $totalWriteBytes += $p.WriteTransferCount
            } catch {}
        }

        $cpuPercent = if ($elapsedMs -gt 0) { ($totalCpuMs / ($elapsedMs * $cpuCount)) * 100.0 } else { 0.0 }

        $samples += [PSCustomObject]@{
            CpuPercent = $cpuPercent
            WorkingSetMB = $totalWs / 1MB
            PrivateMemoryMB = $totalPrivate / 1MB
            MainWsMB = $mainWs / 1MB
            MainPrivateMB = $mainPrivate / 1MB
            WebviewWsMB = $webviewWs / 1MB
            WebviewPrivateMB = $webviewPrivate / 1MB
            PagedMemoryMB = $totalPaged / 1MB
            NonpagedMemoryMB = $totalNonpaged / 1MB
            Handles = $totalHandles
            Threads = $totalThreads
            GdiHandles = $totalGdi
            UserHandles = $totalUser
            ReadBytesMB = $totalReadBytes / 1MB
            WriteBytesMB = $totalWriteBytes / 1MB
            ProcessCount = $procs.Count
        }
    }

    if ($samples.Count -eq 0) { return $null }

    $avgCpu = ($samples | Measure-Object -Property CpuPercent -Average).Average
    $peakCpu = ($samples | Measure-Object -Property CpuPercent -Maximum).Maximum
    $avgWs = ($samples | Measure-Object -Property WorkingSetMB -Average).Average
    $peakWs = ($samples | Measure-Object -Property WorkingSetMB -Maximum).Maximum
    $avgPrivate = ($samples | Measure-Object -Property PrivateMemoryMB -Average).Average
    $peakPrivate = ($samples | Measure-Object -Property PrivateMemoryMB -Maximum).Maximum
    
    $avgMainWs = ($samples | Measure-Object -Property MainWsMB -Average).Average
    $avgMainPrivate = ($samples | Measure-Object -Property MainPrivateMB -Average).Average
    $avgWebviewWs = ($samples | Measure-Object -Property WebviewWsMB -Average).Average
    $avgWebviewPrivate = ($samples | Measure-Object -Property WebviewPrivateMB -Average).Average

    $avgThreads = ($samples | Measure-Object -Property Threads -Average).Average
    $avgHandles = ($samples | Measure-Object -Property Handles -Average).Average
    $avgGdi = ($samples | Measure-Object -Property GdiHandles -Average).Average
    $avgUser = ($samples | Measure-Object -Property UserHandles -Average).Average
    $avgPaged = ($samples | Measure-Object -Property PagedMemoryMB -Average).Average
    $avgNonpaged = ($samples | Measure-Object -Property NonpagedMemoryMB -Average).Average
    $lastProcCount = $samples[-1].ProcessCount

    return [PSCustomObject]@{
        AvgCpuPercent = [math]::Round($avgCpu, 2)
        PeakCpuPercent = [math]::Round($peakCpu, 2)
        AvgWorkingSetMB = [math]::Round($avgWs, 2)
        PeakWorkingSetMB = [math]::Round($peakWs, 2)
        AvgPrivateMemoryMB = [math]::Round($avgPrivate, 2)
        PeakPrivateMemoryMB = [math]::Round($peakPrivate, 2)
        MainProcessWsMB = [math]::Round($avgMainWs, 2)
        MainProcessPrivateMB = [math]::Round($avgMainPrivate, 2)
        WebviewWsMB = [math]::Round($avgWebviewWs, 2)
        WebviewPrivateMB = [math]::Round($avgWebviewPrivate, 2)
        PagedMemoryMB = [math]::Round($avgPaged, 2)
        NonpagedMemoryMB = [math]::Round($avgNonpaged, 2)
        Threads = [int]$avgThreads
        KernelHandles = [int]$avgHandles
        GdiHandles = [int]$avgGdi
        UserHandles = [int]$avgUser
        TreeProcessCount = $lastProcCount
    }
}

Write-Host "================================================================"
Write-Host "      SYNCHRO SYSTEM RESOURCE PROFILER & BENCHMARK"
Write-Host "================================================================"

# Stop any lingering synchro processes
Get-Process synchro* -ErrorAction SilentlyContinue | Stop-Process -Force
Start-Sleep -Seconds 1

# -------------------------------------------------------------
# TEST 1: Полностью открыто меню (Main Window Open & Active)
# -------------------------------------------------------------
Write-Host "`n[1/3] Benchmarking: Полностью открыто меню (Foreground UI)..."
if (Test-Path $settingsPath) {
    $cfg = Get-Content $settingsPath -Raw | ConvertFrom-Json
    $cfg.settings.startMinimized = $false
    $cfg.settings.closeToTray = $true
    $cfg | ConvertTo-Json -Depth 10 | Set-Content $settingsPath -Encoding UTF8
}

$proc1 = Start-Process -FilePath $appPath -WorkingDirectory $appDir -PassThru
Start-Sleep -Seconds 4

$metricsOpen = Measure-ResourceMetrics -ParentPid $proc1.Id -DurationSec 8
Write-Host " -> Done. Total CPU: $($metricsOpen.AvgCpuPercent)%, Total RAM (WS): $($metricsOpen.AvgWorkingSetMB) MB (Synchro core: $($metricsOpen.MainProcessWsMB) MB, WebView: $($metricsOpen.WebviewWsMB) MB), Private Commit: $($metricsOpen.AvgPrivateMemoryMB) MB, Threads: $($metricsOpen.Threads)"

Stop-Process -Id $proc1.Id -Force -ErrorAction SilentlyContinue
Get-Process synchro* -ErrorAction SilentlyContinue | Stop-Process -Force
Start-Sleep -Seconds 1

# -------------------------------------------------------------
# TEST 2: В трее (Tray Daemon Mode, No WebView2)
# -------------------------------------------------------------
Write-Host "`n[2/3] Benchmarking: В трее (Tray Daemon Mode)..."
if (Test-Path $settingsPath) {
    $cfg = Get-Content $settingsPath -Raw | ConvertFrom-Json
    $cfg.settings.startMinimized = $true
    $cfg.settings.closeToTray = $true
    $cfg | ConvertTo-Json -Depth 10 | Set-Content $settingsPath -Encoding UTF8
}

$proc2 = Start-Process -FilePath $appPath -WorkingDirectory $appDir -PassThru
Start-Sleep -Seconds 3

$metricsTray = Measure-ResourceMetrics -ParentPid $proc2.Id -DurationSec 8
Write-Host " -> Done. Total CPU: $($metricsTray.AvgCpuPercent)%, Total RAM (WS): $($metricsTray.AvgWorkingSetMB) MB (Synchro core: $($metricsTray.MainProcessWsMB) MB, WebView: $($metricsTray.WebviewWsMB) MB), Private Commit: $($metricsTray.AvgPrivateMemoryMB) MB, Threads: $($metricsTray.Threads)"

# -------------------------------------------------------------
# TEST 3: Открыта полноэкранная игра (меню закрыто / Game Mode)
# -------------------------------------------------------------
Write-Host "`n[3/3] Benchmarking: Открыта полноэкранная игра (Game Mode Active)..."

$gameForm = New-Object System.Windows.Forms.Form
$gameForm.Text = "Simulated Exclusive Fullscreen 3D Game"
$gameForm.FormBorderStyle = [System.Windows.Forms.FormBorderStyle]::None
$gameForm.WindowState = [System.Windows.Forms.FormWindowState]::Maximized
$gameForm.TopMost = $true
$gameForm.BackColor = [System.Drawing.Color]::Black

$screen = [System.Windows.Forms.Screen]::PrimaryScreen
$gameForm.Bounds = $screen.Bounds
$gameForm.Show()
[Win32]::SetForegroundWindow($gameForm.Handle) | Out-Null

Start-Sleep -Seconds 3

$metricsGame = Measure-ResourceMetrics -ParentPid $proc2.Id -DurationSec 8
Write-Host " -> Done. Total CPU: $($metricsGame.AvgCpuPercent)%, Total RAM (WS): $($metricsGame.AvgWorkingSetMB) MB (Synchro core: $($metricsGame.MainProcessWsMB) MB, WebView: $($metricsGame.WebviewWsMB) MB), Private Commit: $($metricsGame.AvgPrivateMemoryMB) MB, Threads: $($metricsGame.Threads)"

$gameForm.Close()
$gameForm.Dispose()

Stop-Process -Id $proc2.Id -Force -ErrorAction SilentlyContinue
Get-Process synchro* -ErrorAction SilentlyContinue | Stop-Process -Force

if (Test-Path $settingsPath) {
    $cfg = Get-Content $settingsPath -Raw | ConvertFrom-Json
    $cfg.settings.startMinimized = $false
    $cfg | ConvertTo-Json -Depth 10 | Set-Content $settingsPath -Encoding UTF8
}

$result = [PSCustomObject]@{
    OpenMenu = $metricsOpen
    TrayMinimized = $metricsTray
    FullscreenGame = $metricsGame
}

$jsonPath = "$PSScriptRoot\..\release\benchmark-results.json"
$result | ConvertTo-Json -Depth 4 | Set-Content $jsonPath -Encoding UTF8

Write-Host "`n================================================================"
Write-Host "                BENCHMARK RESULTS SUMMARY"
Write-Host "================================================================"
$result | ConvertTo-Json -Depth 4
