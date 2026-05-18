const tasksRoot = document.getElementById('tasks');
const statusChip = document.getElementById('status-chip');
const resultSection = document.getElementById('result');
const resultStatusBadge = document.getElementById('result-status-badge');
const resultStatusValue = document.getElementById('result-status-value');
const doList = document.getElementById('do-list');
const dontList = document.getElementById('dont-list');
const taskColumns = document.getElementById('task-columns');
const langSelect = document.getElementById('lang-select');

let currentLang = 'en';
let latestResponse = null;

const translations = {
  en: {
    appName: 'Recovery Planner',
    heroTitle: 'Manual Heart Rate Input',
    heroDescription: 'Enter your resting heart rate and baseline values, then add tasks to get a recovery-aware daily plan.',
    langLabel: 'Language',
    metricsTitle: 'Metrics',
    metricsSubtitle: 'Quick input for your recovery state.',
    currentRhr: 'Current RHR',
    baselineRhr: 'Baseline RHR',
    hrv: 'HRV (optional)',
    note: 'A higher current RHR than baseline suggests fatigue. Use this input to tune your plan.',
    tasksTitle: 'Tasks',
    tasksSubtitle: 'Add tasks and tag their workload.',
    addTask: 'Add task',
    planButton: 'Plan my day',
    resultEyebrow: 'Recommended plan',
    resultTitle: 'Your daily recovery guidance',
    statusLabel: 'Recovery state',
    doTitle: 'Do',
    dontTitle: "Don't",
    taskBoardTitle: 'Optimized task plan',
    taskBoardDescription: 'Tasks are ordered by what your recovery state recommends.',
    emptyTasks: 'No tasks provided. Add tasks to see the prioritized task plan.',
    removedTask: 'Remove',
    taskTitlePlaceholder: 'Task title',
    taskTitleHelper: 'Describe the task clearly, then choose how mentally and physically demanding it is.',
    cognitiveLabel: 'Cognitive load',
    physicalLabel: 'Physical load',
    noStatusYet: 'No status yet',
    errorTitle: 'Error',
    errorPlanFailed: 'Could not generate a plan',
    serverReturned: 'Server returned',
    networkError: 'Unknown network error',
    cognitive: 'Cognitive',
    physical: 'Physical',
    essential: 'Essential',
    statusFatigued: 'Fatigued',
    statusPrime: 'Prime',
    statusNormal: 'Normal',
  },
  id: {
    appName: 'Perencana Pemulihan',
    heroTitle: 'Input Detak Jantung Manual',
    heroDescription: 'Masukkan detak jantung istirahat dan nilai baseline Anda, lalu tambahkan tugas untuk mendapatkan rencana harian yang memperhatikan pemulihan.',
    langLabel: 'Bahasa',
    metricsTitle: 'Metri',
    metricsSubtitle: 'Masukan cepat untuk kondisi pemulihan Anda.',
    currentRhr: 'RHR Saat Ini',
    baselineRhr: 'RHR Baseline',
    hrv: 'HRV (opsional)',
    note: 'RHR saat ini yang lebih tinggi dari baseline menunjukkan kemungkinan kelelahan. Gunakan nilai ini untuk menyempurnakan rencana.',
    tasksTitle: 'Tugas',
    tasksSubtitle: 'Tambahkan tugas dan beri tag beban kerjanya.',
    addTask: 'Tambah tugas',
    planButton: 'Rencanakan hariku',
    resultEyebrow: 'Rencana rekomendasi',
    resultTitle: 'Panduan pemulihan harian Anda',
    statusLabel: 'Kondisi pemulihan',
    doTitle: 'Yang harus dilakukan',
    dontTitle: 'Yang tidak boleh dilakukan',
    taskBoardTitle: 'Rencana tugas yang dioptimalkan',
    taskBoardDescription: 'Tugas diurutkan berdasarkan kondisi pemulihan Anda.',
    emptyTasks: 'Tidak ada tugas. Tambahkan tugas untuk melihat rencana tugas yang diprioritaskan.',
    removedTask: 'Hapus',
    taskTitlePlaceholder: 'Judul tugas',
    taskTitleHelper: 'Jelaskan tugas dengan jelas, lalu pilih seberapa menuntut secara mental dan fisik.',
    cognitiveLabel: 'Beban kognitif',
    physicalLabel: 'Beban fisik',
    noStatusYet: 'Belum ada status',
    errorTitle: 'Galat',
    errorPlanFailed: 'Rencana gagal dibuat',
    serverReturned: 'Server mengembalikan',
    networkError: 'Kesalahan jaringan tidak diketahui',
    cognitive: 'Kognitif',
    physical: 'Fisik',
    essential: 'Esensial',
    statusFatigued: 'Kelelahan',
    statusPrime: 'Prime',
    statusNormal: 'Normal',
  },
};

const recommendationTranslation = {
  id: {
    'Prioritize recovery and avoid high-strain tasks.': 'Prioritaskan pemulihan dan hindari tugas dengan beban tinggi.',
    'Focus on essential work, short breaks, and hydration.': 'Fokus pada pekerjaan penting, istirahat singkat, dan hidrasi.',
    'Skip heavy workouts today.': 'Lewati latihan berat hari ini.',
    'Limit caffeine and avoid late-night deep work.': 'Batasi kafein dan hindari pekerjaan mendalam larut malam.',
    "Tackle the hardest problems early.": 'Tangani masalah tersulit lebih awal.',
    'Use your recovery window for focused deep work.': 'Gunakan jendela pemulihan Anda untuk pekerjaan fokus.',
    "Don't procrastinate on your top priority tasks.": 'Jangan menunda tugas prioritas utama Anda.',
    'Keep a balanced workload and listen to your energy levels.': 'Pertahankan beban kerja seimbang dan dengarkan tingkat energi Anda.',
    'Start with essential tasks and maintain steady pacing.': 'Mulai dengan tugas penting dan pertahankan ritme yang stabil.',
    'Avoid sudden spikes in physical or mental strain.': 'Hindari lonjakan beban fisik atau mental secara tiba-tiba.',
  },
};

function t(key) {
  return translations[currentLang][key] || translations.en[key] || key;
}

function translateStatus(status) {
  if (currentLang === 'id') {
    if (status === 'Fatigued') return t('statusFatigued');
    if (status === 'Prime') return t('statusPrime');
    if (status === 'Normal') return t('statusNormal');
  }
  return status;
}

function translateRecommendations(items) {
  if (currentLang === 'id') {
    return items.map((item) => recommendationTranslation.id[item] || item);
  }
  return items;
}

function updateLanguageText() {
  document.documentElement.lang = currentLang;
  document.title = `${t('heroTitle')} — ${t('appName')}`;
  document.getElementById('app-name').textContent = t('appName');
  document.getElementById('hero-title').textContent = t('heroTitle');
  document.getElementById('hero-description').textContent = t('heroDescription');
  document.getElementById('lang-label').textContent = t('langLabel');
  document.getElementById('control-panel-title').textContent = t('metricsTitle');
  document.getElementById('control-panel-subtitle').textContent = t('metricsSubtitle');
  document.getElementById('current-rhr-label').textContent = t('currentRhr');
  document.getElementById('baseline-rhr-label').textContent = t('baselineRhr');
  document.getElementById('hrv-label').textContent = t('hrv');
  document.getElementById('note').textContent = t('note');
  document.getElementById('task-panel-title').textContent = t('tasksTitle');
  document.getElementById('task-panel-subtitle').textContent = t('tasksSubtitle');
  document.getElementById('add-task').textContent = t('addTask');
  document.getElementById('plan-button').textContent = t('planButton');
  document.getElementById('result-eyebrow').textContent = t('resultEyebrow');
  document.getElementById('result-title').textContent = t('resultTitle');
  document.getElementById('status-label').textContent = t('statusLabel');
  document.getElementById('do-title').textContent = t('doTitle');
  document.getElementById('dont-title').textContent = t('dontTitle');
  document.getElementById('task-board-title').textContent = t('taskBoardTitle');
  document.getElementById('task-board-description').textContent = t('taskBoardDescription');
}

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
  button.textContent = t('removedTask');
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
    <div class="task-title-group">
      <input placeholder="${t('taskTitlePlaceholder')}" class="task-title" />
      <small class="task-help">${t('taskTitleHelper')}</small>
    </div>
    <label>
      ${t('cognitiveLabel')}
      <select class="cognitive_load">
        <option>Low</option>
        <option>Medium</option>
        <option>High</option>
      </select>
    </label>
    <label>
      ${t('physicalLabel')}
      <select class="physical_load">
        <option>Low</option>
        <option>Medium</option>
        <option>High</option>
      </select>
    </label>
    <label>${t('essential')} <input type="checkbox" class="is_essential"/></label>
  `;
  div.appendChild(createRemoveButton());
  tasksRoot.appendChild(div);
}

function getStatusStyles(status) {
  const normalized = status.toLowerCase();
  if (normalized === 'fatigued') {
    return { color: '#b91c1c', background: '#fee2e2' };
  }
  if (normalized === 'prime') {
    return { color: '#166534', background: '#d1fae5' };
  }
  return { color: '#92400e', background: '#fef3c7' };
}

function createBadge(text, type) {
  const badge = document.createElement('span');
  badge.className = `badge ${type.toLowerCase()}`;
  badge.textContent = text;
  return badge;
}

function renderResult(response) {
  latestResponse = response;
  resultSection.classList.remove('hidden');
  const status = response.status || 'Unknown';
  const styles = getStatusStyles(status);
  const displayStatus = translateStatus(status);

  resultStatusBadge.textContent = displayStatus;
  resultStatusBadge.style.background = styles.background;
  resultStatusBadge.style.color = styles.color;
  resultStatusValue.textContent = displayStatus;
  statusChip.textContent = displayStatus;
  statusChip.style.background = styles.background;
  statusChip.style.color = styles.color;
  statusChip.style.borderColor = styles.background;

  doList.innerHTML = '';
  translateRecommendations(response.do_recommendations).forEach((item) => {
    const li = document.createElement('li');
    li.textContent = item;
    doList.appendChild(li);
  });

  dontList.innerHTML = '';
  translateRecommendations(response.dont_recommendations).forEach((item) => {
    const li = document.createElement('li');
    li.textContent = item;
    dontList.appendChild(li);
  });

  taskColumns.innerHTML = '';
  if (response.optimized_tasks.length === 0) {
    const empty = document.createElement('p');
    empty.className = 'empty-tasks';
    empty.textContent = t('emptyTasks');
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

    const cognitive = createBadge(`${t('cognitive')}: ${task.cognitive_load}`, task.cognitive_load);
    const physical = createBadge(`${t('physical')}: ${task.physical_load}`, task.physical_load);
    meta.appendChild(cognitive);
    meta.appendChild(physical);
    if (task.is_essential) {
      meta.appendChild(createBadge(t('essential'), 'essential'));
    }

    card.append(title, meta);
    taskColumns.appendChild(card);
  });
}

function showError(error) {
  resultSection.classList.remove('hidden');
  resultStatusBadge.textContent = t('errorTitle');
  resultStatusBadge.style.background = '#fee2e2';
  resultStatusBadge.style.color = '#b91c1c';
  resultStatusValue.textContent = t('errorPlanFailed');
  statusChip.textContent = t('errorTitle');
  statusChip.style.background = '#fee2e2';
  statusChip.style.color = '#b91c1c';
  doList.innerHTML = `<li>${error}</li>`;
  dontList.innerHTML = '';
  taskColumns.innerHTML = '';
}

function updateStatusChip() {
  const current = parseInt(document.getElementById('current_rhr').value, 10);
  const baseline = parseInt(document.getElementById('baseline_rhr').value, 10);
  if (!current || !baseline) {
    statusChip.textContent = t('noStatusYet');
    statusChip.style.background = '#e5e7eb';
    statusChip.style.color = '#111827';
    statusChip.style.borderColor = 'transparent';
    return;
  }

  if (current > baseline * 1.1) {
    statusChip.textContent = translateStatus('Fatigued');
    statusChip.style.background = '#fee2e2';
    statusChip.style.color = '#b91c1c';
    statusChip.style.borderColor = '#fecaca';
  } else if (current <= baseline) {
    statusChip.textContent = translateStatus('Prime');
    statusChip.style.background = '#d1fae5';
    statusChip.style.color = '#166534';
    statusChip.style.borderColor = '#a7f3d0';
  } else {
    statusChip.textContent = translateStatus('Normal');
    statusChip.style.background = '#fef3c7';
    statusChip.style.color = '#92400e';
    statusChip.style.borderColor = '#fde68a';
  }
}

function applyTranslations() {
  updateLanguageText();
  document.querySelectorAll('#tasks .task-title').forEach((input) => {
    if (!input.value.trim()) {
      input.placeholder = t('taskTitlePlaceholder');
    }
  });

  if (latestResponse) {
    renderResult(latestResponse);
  }
}

function init() {
  langSelect.value = currentLang;
  langSelect.addEventListener('change', (event) => {
    currentLang = event.target.value;
    applyTranslations();
  });

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
        showError(`${t('serverReturned')} ${res.status}: ${error}`);
        return;
      }
      const json = await res.json();
      latestResponse = json;
      renderResult(json);
    } catch (err) {
      showError(err.message || t('networkError'));
    }
  });

  applyTranslations();
  updateStatusChip();
}

init();
