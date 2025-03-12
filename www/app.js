// Initialize the WebAssembly module
let obadh = null;

// Initialize function to load the WebAssembly module
async function initObadh() {
    try {
        // Import the WebAssembly module
        const wasm = await import('./pkg/obadh_engine.js');
        await wasm.default();
        
        // Create an instance of the WasmEngine
        obadh = new wasm.WasmEngine();
        
        console.log('Obadh Engine initialized successfully');
        
        // Update UI to show engine is ready
        document.getElementById('transliterate-btn').disabled = false;
        document.getElementById('analyze-btn').disabled = false;
        
        // Initialize with any text already in the input
        const input = document.getElementById('roman-input').value.trim();
        if (input) {
            transliterate();
        }
    } catch (err) {
        console.error('Failed to initialize Obadh Engine:', err);
        showNotification('Failed to initialize engine. Please check console for details.', true);
    }
}

// Function to transliterate text
function transliterate() {
    if (!obadh) {
        showNotification('Engine not initialized yet. Please wait...', true);
        return;
    }
    
    const input = document.getElementById('roman-input').value.trim();
    const outputEl = document.getElementById('bengali-output');
    
    try {
        const startTime = performance.now();
        const result = obadh.transliterate(input);
        const endTime = performance.now();
        
        outputEl.textContent = result;
        
        // Log performance
        console.log(`Transliteration completed in ${(endTime - startTime).toFixed(2)}ms`);
    } catch (err) {
        console.error('Transliteration error:', err);
        outputEl.textContent = 'Error during transliteration. Please check console for details.';
        showNotification('Transliteration error', true);
    }
}

// Function to analyze text
function analyze() {
    if (!obadh) {
        showNotification('Engine not initialized yet. Please wait...', true);
        return;
    }
    
    const input = document.getElementById('analyze-input').value.trim();
    const tokensOutput = document.getElementById('tokens-output');
    const phonemesOutput = document.getElementById('phonemes-output');
    const syllablesOutput = document.getElementById('syllables-output');
    const analysisOutput = document.getElementById('analysis-output');
    
    try {
        const startTime = performance.now();
        const analysis = obadh.analyze(input);
        const endTime = performance.now();
        
        // Display tokens
        displayTokens(analysis.tokens, tokensOutput);
        
        // Display phonemes
        displayPhonemes(analysis.phonemes, phonemesOutput);
        
        // Display syllables
        displaySyllables(analysis.syllables, syllablesOutput);
        
        // Display final output
        analysisOutput.textContent = analysis.output;
        
        // Log performance
        console.log(`Analysis completed in ${(endTime - startTime).toFixed(2)}ms`);
    } catch (err) {
        console.error('Analysis error:', err);
        tokensOutput.textContent = '';
        phonemesOutput.textContent = '';
        syllablesOutput.textContent = '';
        analysisOutput.textContent = 'Error during analysis. Please check console for details.';
        showNotification('Analysis error', true);
    }
}

// Display tokens in a user-friendly way
function displayTokens(tokensJson, container) {
    container.innerHTML = '';
    
    try {
        const tokens = JSON.parse(tokensJson);
        
        tokens.forEach(token => {
            const tokenEl = document.createElement('span');
            tokenEl.classList.add('token', `token-${token.type.toLowerCase()}`);
            tokenEl.title = `Type: ${token.type}, Position: ${token.position || 'None'}`;
            tokenEl.textContent = token.text;
            container.appendChild(tokenEl);
        });
    } catch (err) {
        console.error('Error parsing tokens:', err);
        container.textContent = 'Error parsing tokens.';
    }
}

// Display phonemes in a user-friendly way
function displayPhonemes(phonemesJson, container) {
    container.innerHTML = '';
    
    try {
        const phonemes = JSON.parse(phonemesJson);
        
        phonemes.forEach(phoneme => {
            const phonemeEl = document.createElement('span');
            phonemeEl.classList.add('token', `token-${phoneme.type.toLowerCase()}`);
            phonemeEl.title = `Roman: ${phoneme.roman}, Type: ${phoneme.type}`;
            phonemeEl.textContent = phoneme.bengali || phoneme.roman;
            container.appendChild(phonemeEl);
        });
    } catch (err) {
        console.error('Error parsing phonemes:', err);
        container.textContent = 'Error parsing phonemes.';
    }
}

// Display syllables in a user-friendly way
function displaySyllables(syllablesJson, container) {
    container.innerHTML = '';
    
    try {
        const syllables = JSON.parse(syllablesJson);
        
        syllables.forEach(syllable => {
            const syllableEl = document.createElement('div');
            syllableEl.classList.add('syllable');
            
            const components = [];
            if (syllable.consonants) components.push(`Consonants: ${syllable.consonants}`);
            if (syllable.vowel) components.push(`Vowel: ${syllable.vowel}`);
            if (syllable.modifiers) components.push(`Modifiers: ${syllable.modifiers}`);
            if (syllable.standalone) components.push('Standalone');
            if (syllable.reph) components.push('Has Reph');
            if (syllable.yaPhala) components.push('Has Ya-Phala');
            
            syllableEl.textContent = components.join(' | ');
            container.appendChild(syllableEl);
        });
    } catch (err) {
        console.error('Error parsing syllables:', err);
        container.textContent = 'Error parsing syllables.';
    }
}

// Show notification
function showNotification(message, isError = false) {
    // Create notification element if it doesn't exist
    let notification = document.querySelector('.notification');
    if (!notification) {
        notification = document.createElement('div');
        notification.classList.add('notification');
        document.body.appendChild(notification);
    }
    
    // Set notification message and style
    notification.textContent = message;
    notification.classList.toggle('error', isError);
    
    // Show notification
    notification.classList.add('show');
    
    // Hide notification after a few seconds
    setTimeout(() => {
        notification.classList.remove('show');
    }, 3000);
}

// Copy text to clipboard
function copyToClipboard(text) {
    // Use the Clipboard API if available
    if (navigator.clipboard && navigator.clipboard.writeText) {
        navigator.clipboard.writeText(text)
            .then(() => {
                showNotification('Copied to clipboard!');
            })
            .catch(err => {
                console.error('Failed to copy text:', err);
                showNotification('Failed to copy text', true);
            });
    } else {
        // Fallback for older browsers
        const textArea = document.createElement('textarea');
        textArea.value = text;
        textArea.style.position = 'fixed';
        textArea.style.left = '-999999px';
        textArea.style.top = '-999999px';
        document.body.appendChild(textArea);
        textArea.focus();
        textArea.select();
        
        try {
            const successful = document.execCommand('copy');
            if (successful) {
                showNotification('Copied to clipboard!');
            } else {
                showNotification('Failed to copy text', true);
            }
        } catch (err) {
            console.error('Failed to copy text:', err);
            showNotification('Failed to copy text', true);
        }
        
        document.body.removeChild(textArea);
    }
}

// Clear input and output
function clearInput() {
    document.getElementById('roman-input').value = '';
    document.getElementById('bengali-output').textContent = '';
}

// Handle tab switching
function switchTab(tabId) {
    // Update active tab button
    document.querySelectorAll('.tab-btn').forEach(btn => {
        btn.classList.toggle('active', btn.getAttribute('data-tab') === tabId);
    });
    
    // Update active tab content
    document.querySelectorAll('.tab-content').forEach(content => {
        content.classList.toggle('active', content.id === tabId);
    });
}

// Set up event listeners when the DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    // Initialize Obadh Engine
    initObadh();
    
    // Set up tab switching
    document.querySelectorAll('.tab-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            switchTab(btn.getAttribute('data-tab'));
        });
    });
    
    // Transliterate button
    document.getElementById('transliterate-btn').addEventListener('click', transliterate);
    
    // Analyze button
    document.getElementById('analyze-btn').addEventListener('click', analyze);
    
    // Copy button
    document.getElementById('copy-btn').addEventListener('click', () => {
        const output = document.getElementById('bengali-output').textContent;
        if (output) {
            copyToClipboard(output);
        }
    });
    
    // Clear button
    document.getElementById('clear-btn').addEventListener('click', clearInput);
    
    // Live transliteration with debounce
    let debounceTimer;
    document.getElementById('roman-input').addEventListener('input', () => {
        clearTimeout(debounceTimer);
        debounceTimer = setTimeout(transliterate, 300);
    });
    
    // Test buttons
    document.querySelectorAll('.test-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            const text = btn.getAttribute('data-text');
            document.getElementById('roman-input').value = text;
            transliterate();
            switchTab('transliterate');
        });
    });
    
    // Set initial focus
    document.getElementById('roman-input').focus();
});