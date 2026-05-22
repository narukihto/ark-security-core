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
let recommendedMtkLoader = "MTK_DA_V6.bin";
let recommendedQcomLoader = "0000000000200000_27fe520d8259d21a_fhprg.bin"; // 

// Initialize the Frontend Listeners and Event Handlers
document.addEventListener('DOMContentLoaded', () => {
    initSystemListeners();
    setupButtonActions();
});

/**
 * Append a formatted timestamped log message to the terminal UI
 * @param {string} message - The message text to print
 * @param {string} type - Log style type ('info', 'success', 'error', 'warning')
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
            if (chipset.includes('[') && chipset.includes(']')) {
                detectedChipName = chipset.split('[')[1].split(']')[0].toLowerCase();
            } else {
                detectedChipName = "generic";
            }
            
            // Intelligence Mapping for MediaTek and Qualcomm recommended loaders
            if (chipset.toUpperCase().includes('MEDIATEK')) {
                if (detectedChipName.includes("mt68") || detectedChipName.includes("mt678")) {
                    recommendedMtkLoader = "MTK_DA_V6.bin";
                } else {
                    recommendedMtkLoader = "MTK_DA_V5.bin";
                }
                enableTargetButton('MediaTek');
            } 
            else if (chipset.toUpperCase().includes('QUALCOMM') || chipset.toUpperCase().includes('SAMSUNG_QCOM')) {
                // فلترة ذكية لملفات bkerler بناءً على قراءة الهوية المكتشفة
                if (detectedChipName.includes("a528") || detectedChipName.includes("a52s")) {
                    recommendedQcomLoader = "001920E100200000_4A14C27B518909E1_fhprg.elf";
                } else if (detectedChipName.includes("redmi9t")) {
                    recommendedQcomLoader = "001360e100720000_1bebe3863a6781db_fhprg_redmi9t.bin";
                } else if (detectedChipName.includes("poco_f1")) {
                    recommendedQcomLoader = "0008b0e100720000_c924a35f39ce1cdd_fhprg_edlauth_poco_f1.bin";
                } else {
                    recommendedQcomLoader = "0000000000200000_27fe520d8259d21a_fhprg.bin"; // الفولباك العام
                }
                enableTargetButton('Qualcomm');
            } else if (chipset.toUpperCase().includes('SAMSUNG')) {
                enableTargetButton('Samsung');
            } else if (chipset.toUpperCase().includes('APPLE')) {
                enableTargetButton('Apple');
            }

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
    // MediaTek Multi-Stage Execution Pipeline
    btnMtk.addEventListener('click', async () => {
        disableAllExploitButtons();
        logToTerminal(`Initiating dynamic multi-stage execution pipeline for MTK: ${detectedChipName}...`, 'info');
        
        try {
            logToTerminal(`[Stage 1/3] Injecting BROM hardware payload vector...`, 'info');
            const bypassResult = await invoke('launch_mtk_bypass', { chipName: detectedChipName });
            logToTerminal(`Bypass Success: ${bypassResult}`, 'success');
            
            logToTerminal(`[Stage 2/3] Uploading Download Agent: ${recommendedMtkLoader}...`, 'info');
            const loaderResult = await invoke('upload_mtk_loader', { daFilename: recommendedMtkLoader });
            logToTerminal(`Loader Injected: ${loaderResult}`, 'success');
            
            logToTerminal(`[Stage 3/3] Commencing final MTK partition formatting routine...`, 'info');
            const finalResult = await invoke('wipe_mtk_frp');
            logToTerminal(`PIPELINE COMPLETE: ${finalResult}`, 'success');
            
        } catch (error) {
            logToTerminal(`PIPELINE CRASH: ${error}`, 'error');
        }
    });

    // Qualcomm EDL 9008 Multi-Stage Execution Pipeline (التحديث الجديد المطابق للمراحل الثلاث)
    btnQualcomm.addEventListener('click', async () => {
        disableAllExploitButtons();
        logToTerminal(`Initiating dynamic multi-stage execution pipeline for Qualcomm EDL 9008...`, 'info');
        
        try {
            logToTerminal(`[Stage 1/3] Establishing synchronous EDL handshake channel...`, 'info');
            const bypassResult = await invoke('launch_qcom_bypass');
            logToTerminal(`Handshake Active: ${bypassResult}`, 'success');
            
            logToTerminal(`[Stage 2/3] Streaming bkerler Firehose Programmer: ${recommendedQcomLoader}...`, 'info');
            const loaderResult = await invoke('upload_qcom_loader', { loaderFilename: recommendedQcomLoader });
            logToTerminal(`Firehose Injected: ${loaderResult}`, 'success');
            
            logToTerminal(`[Stage 3/3] Executing structural partition storage layout wipe...`, 'info');
            const finalResult = await invoke('wipe_qcom_frp');
            logToTerminal(`PIPELINE COMPLETE: ${finalResult}`, 'success');
            
        } catch (error) {
            logToTerminal(`QUALCOMM PIPELINE CRASH: ${error}`, 'error');
        }
    });

    // Fallback bindings for separate hardware protocols
    btnSamsung.addEventListener('click', () => triggerExploitExecution('execute_protocol_handshake', { platform: 'samsung' }));
    btnApple.addEventListener('click', () => triggerExploitExecution('execute_protocol_handshake', { platform: 'apple' }));

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
 * @param {object} args - Arguments to pass to the command
 */
async function triggerExploitExecution(commandName, args = {}) {
    disableAllExploitButtons();
    logToTerminal(`Initiating data beam transmission via: ${commandName}...`, 'info');
    
    try {
        const result = await invoke(commandName, args);
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
