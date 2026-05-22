// src/main.js

// Import Tauri core invocation API for high-speed IPC communication
const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

// DOM Elements Selection
const statusText = document.getElementById('status-text');
const pulseDot = document.querySelector('.pulse-dot');
const chipsetType = document.getElementById('chipset-type');
const hardwareId = document.getElementById('hardware-id');
const terminalOutput = document.getElementById('terminal-output');

// Action Buttons
const btnMtk = document.getElementById('btn-mtk');
const btnQualcomm = document.getElementById('btn-qualcomm');
const btnSamsung = document.getElementById('btn-samsung');
const btnApple = document.getElementById('btn-apple');
const btnBlackStone = document.getElementById('btn-black-stone');
const btnClearLogs = document.getElementById('btn-clear-logs');

// Global state to track detected hardware specs dynamically
let detectedChipName = "generic";
let recommendedLoader = "MTK_DA_V6.bin";

// Initialize the Frontend Listeners and Event Handlers
document.addEventListener('DOMContentLoaded', () => {
    initSystemListeners();
    setupButtonActions();
});

/**
 * Append a formatted timestamped log message to the terminal UI
 * @param {string} message - The message text to print
 * @param {string} type - Log style type ('info', 'success', 'error', 'warning', 'success')
 */
function logToTerminal(message, type = 'info') {
    const timestamp = new Date().toLocaleTimeString();
    const logLine = document.createElement('div');
    logLine.className = `log-line ${type}`;
    logLine.innerText = `[${timestamp}] ${message}`;
    
    terminalOutput.appendChild(logLine);
    terminalOutput.scrollTop = terminalOutput.scrollHeight;
}

/**
 * Update the visual illumination of the 10 mirrors based on current active index
 * @param {number} activeMirrorIndex - Current state position (1 to 10)
 */
function updateMirrorUI(activeMirrorIndex) {
    for (let i = 1; i <= 10; i++) {
        const mirrorNode = document.getElementById(`m${i}`);
        if (mirrorNode) {
            if (i <= activeMirrorIndex) {
                mirrorNode.className = 'mirror-node active';
            } else {
                mirrorNode.className = 'mirror-node';
            }
        }
    }
}

/**
 * Establishes real-time listeners for events beamed from the Rust backend kernel
 */
async function initSystemListeners() {
    logToTerminal('Initializing real-time asynchronous kernel event listeners...', 'info');

    // Listen for global device detection events from Rust background thread
    await listen('device-state-changed', (event) => {
        const { status, vid_pid, chipset, active_mirror } = event.payload;

        // Dynamic State UI Blending
        statusText.innerText = status.toUpperCase();
        hardwareId.innerText = vid_pid;
        chipsetType.innerText = chipset.toUpperCase();
        
        // Synchronize state mirror visuals
        updateMirrorUI(active_mirror);

        // Adjust pulse indicator state
        pulseDot.className = 'pulse-dot';
        if (chipset !== 'NONE') {
            pulseDot.classList.add('connected');
            
            // Extract core details if provided by the hardware registration event
            // Example: "MediaTek [MT6765]"
            if (chipset.includes('[') && chipset.includes(']')) {
                detectedChipName = chipset.split('[')[1].split(']')[0].toLowerCase();
            } else {
                detectedChipName = "generic";
            }
            
            // Intelligence Mapping for recommended DA Loader versions
            if (detectedChipName.includes("mt68") || detectedChipName.includes("mt678")) {
                recommendedLoader = "MTK_DA_V6.bin";
            } else {
                recommendedLoader = "MTK_DA_V5.bin";
            }

            enableTargetButton('MediaTek'); // Force selection fallback during tests
            logToTerminal(`Hardware Event: ${chipset} detected [${vid_pid}] at Mirror ${active_mirror}`, 'warning');
        } else {
            pulseDot.classList.add('idle');
            disableAllExploitButtons();
        }
    });

    // Listen for critical error logs or global rollback actions (Brown/Black Stone triggers)
    await listen('stone-security-alert', (event) => {
        const { message, severity, target_mirror } = event.payload;
        logToTerminal(`ALERT: ${message}`, severity);
        updateMirrorUI(target_mirror);
    });

    // Listen for standard processing progression logs
    await listen('execution-progress', (event) => {
        const { message, step_index } = event.payload;
        logToTerminal(message, 'info');
        updateMirrorUI(step_index);
    });
}

/**
 * Configure DOM interaction listeners for operational control buttons
 */
function setupButtonActions() {
    // Structural Multi-Stage Pipeline Execution for MediaTek Platform
    btnMtk.addEventListener('click', async () => {
        disableAllExploitButtons();
        logToTerminal(`Initiating dynamic multi-stage execution pipeline for chip: ${detectedChipName}...`, 'info');
        
        try {
            // Stage 1: Execute BROM Security Bypass
            logToTerminal(`[Stage 1/3] Injecting specific hardware payload vector...`, 'info');
            const bypassResult = await invoke('launch_mtk_bypass', { chipName: detectedChipName });
            logToTerminal(`Bypass Success: ${bypassResult}`, 'success');
            
            // Stage 2: Target and Upload DA Loader Agent
            logToTerminal(`[Stage 2/3] Uploading matching Download Agent: ${recommendedLoader}...`, 'info');
            const loaderResult = await invoke('upload_mtk_loader', { daFilename: recommendedLoader });
            logToTerminal(`Loader Injected: ${loaderResult}`, 'success');
            
            // Stage 3: Wipe persistent locks / Clear FRP Layout
            logToTerminal(`[Stage 3/3] Commencing final partition formatting routine...`, 'info');
            const finalResult = await invoke('wipe_mtk_frp');
            logToTerminal(`PIPELINE COMPLETE: ${finalResult}`, 'success');
            
        } catch (error) {
            logToTerminal(`PIPELINE CRASH: ${error}`, 'error');
        }
    });

    // Fallback bindings for separate hardware protocols
    btnQualcomm.addEventListener('click', () => triggerExploitExecution('execute_qualcomm_unlock'));
    btnSamsung.addEventListener('click', () => triggerExploitExecution('execute_samsung_frp'));
    btnApple.addEventListener('click', () => triggerExploitExecution('execute_apple_pongo'));

    // The Black Stone Gate - Global Emergency Interruption
    btnBlackStone.addEventListener('click', async () => {
        try {
            logToTerminal('CRITICAL: Black Stone Intercept pressed manually by operator!', 'error');
            await invoke('trigger_black_stone_lock');
        } catch (err) {
            logToTerminal(`Failed to transmit Black Stone signal: ${err}`, 'error');
        }
    });

    // Terminal clear function
    btnClearLogs.addEventListener('click', () => {
        terminalOutput.innerHTML = '';
        logToTerminal('Terminal logs wiped.', 'info');
    });
}

/**
 * Invokes standard single-stage low-level execution payload asynchronously within Rust core
 * @param {string} commandName - The backend core invocation target string
 */
async function triggerExploitExecution(commandName) {
    disableAllExploitButtons();
    logToTerminal(`Initiating data beam transmission via: ${commandName}...`, 'info');
    
    try {
        const result = await invoke(commandName);
        logToTerminal(`SUCCESS: ${result}`, 'success');
    } catch (error) {
        logToTerminal(`EXECUTION FAILURE: ${error}`, 'error');
    }
}

function enableTargetButton(chipset) {
    disableAllExploitButtons();
    if (chipset.includes('MediaTek')) btnMtk.disabled = false;
    if (chipset.includes('Qualcomm')) btnQualcomm.disabled = false;
    if (chipset.includes('Samsung')) btnSamsung.disabled = false;
    if (chipset.includes('Apple')) btnApple.disabled = false;
}

function disableAllExploitButtons() {
    btnMtk.disabled = true;
    btnQualcomm.disabled = true;
    btnSamsung.disabled = true;
    btnApple.disabled = true;
}
