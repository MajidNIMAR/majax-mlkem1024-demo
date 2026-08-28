const runButton = document.querySelector('#run');
const messageInput = document.querySelector('#message');
const progress = document.querySelector('#progress');
const resultPanel = document.querySelector('#result');
const statusBadge = document.querySelector('#status');
const statusTitle = document.querySelector('#status-title');
const checksContainer = document.querySelector('#checks');
const rawOutput = document.querySelector('#raw');

const CHECK_LABELS = {
  algorithm_identifier_correct: 'Correct ML-KEM-1024 algorithm identifier',
  standardized_object_dimensions: 'Standardized object dimensions verified',
  encapsulation_decapsulation_reciprocal: 'Reciprocal secret after decapsulation',
  hybrid_message_round_trip: 'Protected message recovered',
  altered_ciphertext_secret_diverges: 'Altered ciphertext detected',
  altered_ciphertext_rejected_by_authenticated_envelope: 'Altered envelope rejected',
};

function milliseconds(value) {
  return `${Number(value).toFixed(3)} ms`;
}

function render(result) {
  const passed = result.overall_result === 'PASS';
  statusBadge.textContent = result.overall_result;
  statusBadge.classList.toggle('fail', !passed);
  statusTitle.textContent = passed ? 'Checks passed' : 'A check failed';
  checksContainer.replaceChildren();
  for (const [name, value] of Object.entries(result.checks || {})) {
    const item = document.createElement('div');
    item.className = `check${value ? '' : ' fail'}`;
    item.textContent = CHECK_LABELS[name] || name;
    checksContainer.append(item);
  }
  document.querySelector('#time-keygen').textContent = milliseconds(result.timings_ms.key_generation);
  document.querySelector('#time-enc').textContent = milliseconds(result.timings_ms.encapsulation);
  document.querySelector('#time-dec').textContent = milliseconds(result.timings_ms.decapsulation);
  document.querySelector('#time-total').textContent = milliseconds(result.timings_ms.total);
  rawOutput.textContent = JSON.stringify(result, null, 2);
  resultPanel.hidden = false;
  resultPanel.scrollIntoView({ behavior: 'smooth', block: 'start' });
}

runButton.addEventListener('click', async () => {
  runButton.disabled = true;
  progress.hidden = false;
  resultPanel.hidden = true;
  try {
    const response = await fetch('/api/run', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ message: messageInput.value }),
    });
    const result = await response.json();
    if (!response.ok || !result.overall_result) {
      throw new Error(result.error || 'demonstration-failed');
    }
    render(result);
  } catch {
    render({
      overall_result: 'FAIL',
      checks: { demonstration_execution: false },
      timings_ms: { key_generation: 0, encapsulation: 0, decapsulation: 0, total: 0 },
      error: 'The demonstration could not be executed.',
    });
  } finally {
    progress.hidden = true;
    runButton.disabled = false;
  }
});
