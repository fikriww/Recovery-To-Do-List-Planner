const tasksRoot = document.getElementById('tasks');
const statusChip = document.getElementById('status-chip');
const resultSection = document.getElementById('result');
const resultStatusBadge = document.getElementById('result-status-badge');
const resultStatusValue = document.getElementById('result-status-value');
const doList = document.getElementById('do-list');
const dontList = document.getElementById('dont-list');
const taskColumns = document.getElementById('task-columns');

function generateTaskId() {
  if (window.crypto && window.crypto.randomUUID) {
    return window.crypto.randomUUID();
  }
  return `task-${Date.now()}-${Math.floor(Math.random() * 10000)}`;
}

function formatTitle(value) {
  return value
    .trim()
    .toLowerCase()
    .split(/\s+/)
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');
}

function getTasksFromDOM() {
  const tasks = [];
  document.querySelectorAll('#tasks .task').forEach((el) => {
    const titleRaw = el.querySelector('.task-title').value.trim();
    if (!titleRaw) {
      return;
    }
    const id = generateTaskId();
    const title = titleRaw;
    const cognitive_load = el.querySelector('.cognitive_load').value;
    const physical_load = el.querySelector('.physical_load').value;
    const is_essential = el.querySelector('.is_essential').checked;
    tasks.push({ id, title, cognitive_load, physical_load, is_essential });
  });
  return tasks;
}

function createRemoveButton() {
  const button = document.createElement('button');
  button.type = 'button';
  button.textContent = 'Hapus';
  button.className = 'secondary remove-task';
  button.addEventListener('click', (event) => {
    event.currentTarget.closest('.task').remove();
  });
  return button;
}

function addTaskRow() {
  const div = document.createElement('div');
  div.className = 'task';
  div.innerHTML = `
    <input placeholder="Judul tugas" class="task-title" />
    <select class="cognitive_load">
      <option>Low</option>
      <option>Medium</option>
      <option>High</option>
    </select>
    <select class="physical_load">
      <option>Low</option>
      <option>Medium</option>
      <option>High</option>
    </select>
    <label>Esensial <input type="checkbox" class="is_essential"/></label>
  `;
  div.appendChild(createRemoveButton());
  tasksRoot.appendChild(div);
}

function getStatusStyles(status) {
  const normalized = status.toLowerCase();
  if (normalized === 'fatigued') {
    return { color: '#b91c1c', background: '#fee2e2', text: 'Fatigued' };
  }
  if (normalized === 'prime') {
    return { color: '#166534', background: '#d1fae5', text: 'Prime' };
  }
  return { color: '#92400e', background: '#fef3c7', text: status };
}

function createBadge(text, type) {
  const badge = document.createElement('span');
  badge.className = `badge ${type.toLowerCase()}`;
  badge.textContent = text;
  return badge;
}

function renderResult(response) {
  resultSection.classList.remove('hidden');
  const status = response.status || 'Unknown';
  const styles = getStatusStyles(status);
  resultStatusBadge.textContent = status;
  resultStatusBadge.style.background = styles.background;
  resultStatusBadge.style.color = styles.color;
  resultStatusValue.textContent = status;
  statusChip.textContent = status;
  statusChip.style.background = styles.background;
  statusChip.style.color = styles.color;
  statusChip.style.borderColor = styles.background;

  doList.innerHTML = '';
  response.do_recommendations.forEach((item) => {
    const li = document.createElement('li');
    li.textContent = item;
    doList.appendChild(li);
  });

  dontList.innerHTML = '';
  response.dont_recommendations.forEach((item) => {
    const li = document.createElement('li');
    li.textContent = item;
    dontList.appendChild(li);
  });

  taskColumns.innerHTML = '';
  if (response.optimized_tasks.length === 0) {
    const empty = document.createElement('p');
    empty.className = 'empty-tasks';
    empty.textContent = 'Tidak ada tugas. Tambahkan tugas untuk melihat rencana tugas yang diprioritaskan.';
    taskColumns.appendChild(empty);
    return;
  }

  response.optimized_tasks.forEach((task) => {
    const card = document.createElement('div');
    card.className = 'task-card';
    const title = document.createElement('h4');
    title.textContent = formatTitle(task.title);
    const meta = document.createElement('div');
    meta.className = 'task-meta';

    const cognitive = createBadge(`Kognitif: ${task.cognitive_load}`, task.cognitive_load);
    const physical = createBadge(`Fisik: ${task.physical_load}`, task.physical_load);
    meta.appendChild(cognitive);
    meta.appendChild(physical);
    if (task.is_essential) {
      meta.appendChild(createBadge('Esensial', 'essential'));
    }

    card.append(title, meta);
    taskColumns.appendChild(card);
  });
}

function updateStatusChip() {
  const current = parseInt(document.getElementById('current_rhr').value, 10);
  const baseline = parseInt(document.getElementById('baseline_rhr').value, 10);
  if (!current || !baseline) {
    statusChip.textContent = 'Belum ada status';
    statusChip.style.background = '#e5e7eb';
    statusChip.style.color = '#111827';
    statusChip.style.borderColor = 'transparent';
    return;
  }

  if (current > baseline * 1.1) {
    statusChip.textContent = 'Kelelahan';
    statusChip.style.background = '#fee2e2';
    statusChip.style.color = '#b91c1c';
    statusChip.style.borderColor = '#fecaca';
  } else if (current <= baseline) {
    statusChip.textContent = 'Prime';
    statusChip.style.background = '#d1fae5';
    statusChip.style.color = '#166534';
    statusChip.style.borderColor = '#a7f3d0';
  } else {
    statusChip.textContent = 'Normal';
    statusChip.style.background = '#fef3c7';
    statusChip.style.color = '#92400e';
    statusChip.style.borderColor = '#fde68a';
  }
}

function showError(error) {
  resultSection.classList.remove('hidden');
  resultStatusBadge.textContent = 'Galat';
  resultStatusBadge.style.background = '#fee2e2';
  resultStatusBadge.style.color = '#b91c1c';
  resultStatusValue.textContent = 'Rencana gagal dibuat';
  statusChip.textContent = 'Galat';
  statusChip.style.background = '#fee2e2';
  statusChip.style.color = '#b91c1c';
  doList.innerHTML = `<li>${error}</li>`;
  dontList.innerHTML = '';
  taskColumns.innerHTML = '';
}

function init() {
  document.getElementById('add-task').addEventListener('click', () => {
    addTaskRow();
  });

  document.getElementById('current_rhr').addEventListener('input', updateStatusChip);
  document.getElementById('baseline_rhr').addEventListener('input', updateStatusChip);

  document.getElementById('metrics-form').addEventListener('submit', async (e) => {
    e.preventDefault();
    const current_rhr = parseInt(document.getElementById('current_rhr').value, 10);
    const baseline_rhr = parseInt(document.getElementById('baseline_rhr').value, 10);
    const hrvVal = document.getElementById('hrv').value;
    const hrv = hrvVal === '' ? null : parseFloat(hrvVal);

    const payload = {
      morning_metrics: { current_rhr, baseline_rhr, hrv },
      tasks: getTasksFromDOM(),
    };

    try {
      const res = await fetch('/api/v1/plan-day', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      });
      if (!res.ok) {
        const error = await res.text();
        showError(`Server mengembalikan ${res.status}: ${error}`);
        return;
      }
      const json = await res.json();
      renderResult(json);
    } catch (err) {
      showError(err.message || 'Kesalahan jaringan tidak diketahui');
    }
  });

  updateStatusChip();
}

init();
