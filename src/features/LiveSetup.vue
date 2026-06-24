<template>
  <div class="live-setup-container">
    <div class="glass-panel main-content">
      <header class="setup-header">
        <h1>TikTok Live Console</h1>
        <p>Quản lý và khởi tạo phiên livestream mới</p>
      </header>

      <div class="setup-grid">
        <!-- Cột Form Tạo Phiên -->
        <div class="form-section">
          <h2>Tạo Phiên Mới</h2>
          <form @submit.prevent="handleCreateSession" class="modern-form">
            <div class="form-group">
              <label>Nền Tảng</label>
              <div class="custom-select" tabindex="0" @blur="platformOpen = false" @click="platformOpen = !platformOpen">
                <div class="select-trigger">
                  <span>{{ form.platform === 'tiktok' ? 'TikTok' : 'Shopee' }}</span>
                  <svg class="chevron" :class="{ open: platformOpen }" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>
                </div>
                <div class="select-dropdown" v-show="platformOpen">
                  <div class="select-option" :class="{ active: form.platform === 'tiktok' }" @click.stop="selectPlatform('tiktok')">
                    TikTok
                  </div>
                  <div class="select-option" :class="{ active: form.platform === 'shopee' }" @click.stop="selectPlatform('shopee')">
                    Shopee
                  </div>
                </div>
              </div>
            </div>
            <div class="form-group">
              <label>Tên Phiên (Tùy chọn)</label>
              <input v-model="form.title" type="text" placeholder="VD: Khui kiện si Hàn 24/06" class="input-field"/>
            </div>
            <div class="form-group" :class="{ 'has-error': errors.platformSessionId }">
              <label>Username / Link Live <span class="required-star">*</span></label>
              <input 
                v-model="form.platformSessionId" 
                @input="errors.platformSessionId = false"
                type="text" 
                placeholder="@username hoặc link" 
                class="input-field"
              />
              <span v-if="errors.platformSessionId" class="error-message">Trường này là bắt buộc. Vui lòng nhập Username hoặc Link Live.</span>
            </div>
            
            <button type="submit" class="btn-primary" :disabled="loading">
              <span v-if="loading" class="spinner"></span>
              <span v-else>Khởi Tạo Phiên</span>
            </button>
          </form>
        </div>

        <!-- Cột Danh sách phiên cũ -->
        <div class="history-section">
          <h2>Các Phiên Gần Đây</h2>
          <div class="session-list" v-if="sessions.length > 0">
            <div v-for="session in sessions" :key="session.id" class="session-card">
              <div class="session-info">
                <h3>{{ session.title || 'Phiên Live Không Tên' }}</h3>
                <span class="status-badge" :class="session.status">{{ session.status }}</span>
                <p class="date">{{ new Date(session.createdAt).toLocaleString('vi-VN') }}</p>
              </div>
              <div class="session-actions">
                <button v-if="session.status === 'draft'" @click="handleStartSession(session.id)" class="btn-start">
                  Bắt Đầu
                </button>
                <button v-else class="btn-secondary">Xem Lịch Sử</button>
                <button @click.stop.prevent="handleDeleteSession(session.id, session.title)" class="btn-delete-icon" title="Xóa phiên">
                  <X :size="18" />
                </button>
              </div>
            </div>
          </div>
          <div v-else class="empty-state">
            <p>Chưa có phiên live nào.</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Confirm Delete Modal -->
    <div v-if="sessionToDelete" class="modal-overlay" @click.self="sessionToDelete = null">
      <div class="modal-content glass-panel">
        <h3>Xóa Phiên Live</h3>
        <p>Bạn có chắc chắn muốn xóa phiên live "<strong>{{ sessionToDelete.title || 'Không Tên' }}</strong>" không?</p>
        <p class="text-danger">Mọi dữ liệu liên quan sẽ bị xóa sạch và không thể khôi phục.</p>
        <div class="modal-actions">
          <button @click="sessionToDelete = null" class="modal-btn-cancel">Hủy</button>
          <button @click="executeDelete" class="modal-btn-danger">
            <Trash2 :size="18" /> Xóa Vĩnh Viễn
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import type { LiveSession, CreateSessionInput } from '../types/session';
import { X, Trash2 } from 'lucide-vue-next';

const router = useRouter();

const sessions = ref<LiveSession[]>([]);
const loading = ref(false);

const form = ref<CreateSessionInput>({
  platform: 'tiktok',
  title: '',
  platformSessionId: ''
});

const platformOpen = ref(false);

const selectPlatform = (platform: string) => {
  form.value.platform = platform;
  platformOpen.value = false;
};

const fetchSessions = async () => {
  try {
    sessions.value = await invoke<LiveSession[]>('get_sessions');
  } catch (e) {
    console.error("Lỗi khi tải danh sách phiên:", e);
  }
};

const errors = ref({
  platformSessionId: false
});

const handleCreateSession = async () => {
  if (!form.value.platformSessionId.trim()) {
    errors.value.platformSessionId = true;
    return;
  }
  errors.value.platformSessionId = false;

  loading.value = true;
  try {
    const newSession = await invoke<LiveSession>('create_session', { input: form.value });
    sessions.value.unshift(newSession);
    form.value.title = '';
    form.value.platformSessionId = '';
  } catch (e) {
    console.error("Lỗi khi tạo phiên:", e);
  } finally {
    loading.value = false;
  }
};

const handleStartSession = async (id: string) => {
  try {
    await invoke('start_session', { id });
    await fetchSessions();
    await router.push('/live-console');
  } catch (e) {
    console.error("Lỗi khi bắt đầu phiên:", e);
  }
};

const sessionToDelete = ref<{id: string, title: string | null} | null>(null);

const handleDeleteSession = (id: string, title: string | null) => {
  sessionToDelete.value = { id, title };
};

const executeDelete = async () => {
  if (!sessionToDelete.value) return;
  const id = sessionToDelete.value.id;
  
  try {
    await invoke('delete_session', { id });
    await fetchSessions();
    sessionToDelete.value = null;
  } catch (e) {
    console.error("Lỗi khi xóa phiên:", e);
    alert(`Có lỗi xảy ra khi xóa phiên: ${e}`);
  }
};

onMounted(() => {
  fetchSessions();
});
</script>



<style scoped>
.live-setup-container {
  display: flex;
  justify-content: center;
  padding: 1rem 2rem 2rem 2rem;
  height: 100%;
  box-sizing: border-box;
  overflow-y: auto;
}

.glass-panel {
  background: var(--surface);
  backdrop-filter: blur(16px);
  border: 1px solid var(--border);
  border-radius: 24px;
  width: 100%;
  max-width: 1000px;
  padding: 2.5rem;
  display: flex;
  flex-direction: column;
}

.setup-header {
  text-align: center;
  margin-bottom: 3rem;
}

.setup-header h1 {
  margin: 0;
  font-size: 2.5rem;
  background: linear-gradient(135deg, #60a5fa, #c084fc);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.setup-header p {
  color: var(--text-muted);
  font-size: 1.1rem;
  margin-top: 0.5rem;
}

.setup-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 3rem;
}

.form-section h2, .history-section h2 {
  font-size: 1.5rem;
  margin-top: 0;
  margin-bottom: 1.5rem;
  color: var(--text-main);
}

.modern-form {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-group label {
  font-size: 0.9rem;
  color: var(--text-muted);
  font-weight: 500;
}

.input-field {
  background: rgba(15, 23, 42, 0.6);
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 0.8rem 1rem;
  color: var(--text-main);
  font-size: 1rem;
  transition: all 0.2s ease;
}

.input-field:focus {
  outline: none;
  border-color: var(--primary);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.2);
}

.required-star {
  color: #ef4444;
  margin-left: 2px;
}

.form-group.has-error .input-field {
  border-color: #ef4444;
  background: rgba(239, 68, 68, 0.05);
}

.form-group.has-error .input-field:focus {
  box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.2);
}

.error-message {
  color: #ef4444;
  font-size: 0.85rem;
  margin-top: 0.3rem;
  animation: slideIn 0.2s ease-out;
}

/* Custom Select */
.custom-select {
  position: relative;
  width: 100%;
  outline: none;
}

.select-trigger {
  background: rgba(15, 23, 42, 0.6);
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 0.8rem 1rem;
  color: var(--text-main);
  font-size: 1rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  cursor: pointer;
  transition: all 0.2s ease;
}

.custom-select:focus .select-trigger {
  border-color: var(--primary);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.2);
}

.chevron {
  transition: transform 0.2s ease;
  color: var(--text-muted);
}

.chevron.open {
  transform: rotate(180deg);
}

.select-dropdown {
  position: absolute;
  top: calc(100% + 8px);
  left: 0;
  right: 0;
  background: #1e293b;
  border: 1px solid var(--border);
  border-radius: 12px;
  box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.5);
  z-index: 50;
  overflow: hidden;
}

.select-option {
  padding: 0.8rem 1rem;
  cursor: pointer;
  transition: background 0.2s;
  color: var(--text-main);
}

.select-option:hover {
  background: rgba(59, 130, 246, 0.1);
}

.select-option.active {
  background: rgba(59, 130, 246, 0.2);
  color: var(--primary);
  font-weight: 500;
}

.btn-primary {
  background: var(--primary);
  color: white;
  border: none;
  padding: 1rem;
  border-radius: 12px;
  font-size: 1.1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  margin-top: 1rem;
}

.btn-primary:hover:not(:disabled) {
  background: var(--primary-hover);
  transform: translateY(-2px);
}

.btn-primary:disabled {
  opacity: 0.7;
  cursor: not-allowed;
}

.session-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  max-height: 400px;
  overflow-y: auto;
  padding-right: 0.5rem;
}

.session-list::-webkit-scrollbar {
  width: 6px;
}

.session-list::-webkit-scrollbar-thumb {
  background: var(--border);
  border-radius: 10px;
}

.session-card {
  background: rgba(15, 23, 42, 0.4);
  border: 1px solid var(--border);
  border-radius: 16px;
  padding: 1.2rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  transition: all 0.2s ease;
}

.session-card:hover {
  background: var(--surface-hover);
}

.session-info h3 {
  margin: 0 0 0.5rem 0;
  font-size: 1.1rem;
}

.status-badge {
  font-size: 0.75rem;
  padding: 0.2rem 0.6rem;
  border-radius: 99px;
  text-transform: uppercase;
  font-weight: 700;
  letter-spacing: 0.05em;
}

.status-badge.draft { background: rgba(148, 163, 184, 0.2); color: #94a3b8; }
.status-badge.running { background: rgba(34, 197, 94, 0.2); color: #4ade80; }
.status-badge.ended { background: rgba(248, 113, 113, 0.2); color: #f87171; }

.date {
  font-size: 0.85rem;
  color: var(--text-muted);
  margin: 0.5rem 0 0 0;
}

.session-actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.btn-start {
  background: #22c55e;
  color: white;
  border: none;
  padding: 0.6rem 1.2rem;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-start:hover {
  background: #16a34a;
}

.btn-secondary {
  background: transparent;
  color: var(--primary);
  border: 1px solid var(--primary);
  padding: 0.6rem 1.2rem;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
}

.btn-delete-icon {
  background: transparent;
  color: var(--text-muted);
  border: none;
  padding: 0.5rem;
  border-radius: 8px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transform: translateX(10px);
  transition: all 0.2s ease;
}

.session-card:hover .btn-delete-icon {
  opacity: 1;
  transform: translateX(0);
}

.btn-delete-icon:hover {
  background: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.empty-state {
  text-align: center;
  color: var(--text-muted);
  padding: 3rem 0;
  border: 1px dashed var(--border);
  border-radius: 16px;
}

/* Modal Styles */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: fadeIn 0.2s ease-out;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.modal-content {
  background: var(--surface);
  border: 1px solid var(--border);
  padding: 2rem;
  max-width: 400px;
  width: 90%;
  border-radius: 20px;
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.5);
  animation: slideUp 0.3s ease-out;
}

@keyframes slideUp {
  from { opacity: 0; transform: translateY(20px); }
  to { opacity: 1; transform: translateY(0); }
}

.modal-content h3 {
  margin-top: 0;
  color: var(--text-main);
}

.text-danger {
  color: #ef4444;
  font-size: 0.9rem;
  margin-top: 0.5rem;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  margin-top: 2rem;
}

.modal-btn-cancel {
  background: rgba(255, 255, 255, 0.05);
  color: var(--text-main);
  border: 1px solid var(--border);
  padding: 0.8rem 1.5rem;
  border-radius: 12px;
  font-weight: 600;
  font-size: 0.95rem;
  cursor: pointer;
  transition: all 0.2s ease;
}

.modal-btn-cancel:hover {
  background: rgba(255, 255, 255, 0.1);
}

.modal-btn-danger {
  background: linear-gradient(135deg, #ef4444, #dc2626);
  color: white;
  border: none;
  padding: 0.8rem 1.5rem;
  border-radius: 12px;
  font-weight: 600;
  font-size: 0.95rem;
  cursor: pointer;
  box-shadow: 0 4px 12px rgba(239, 68, 68, 0.3);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.modal-btn-danger:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(239, 68, 68, 0.4);
  background: linear-gradient(135deg, #f87171, #ef4444);
}

.modal-btn-danger:active {
  transform: translateY(0);
  box-shadow: 0 2px 8px rgba(239, 68, 68, 0.3);
}
</style>
