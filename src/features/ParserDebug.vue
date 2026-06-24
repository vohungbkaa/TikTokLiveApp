<template>
  <div class="parser-debug">
    <div class="glass-panel main-content">
      <header class="header-section">
        <div class="header-text">
          <h1>Công cụ Debug Parser</h1>
          <p>Kiểm tra thuật toán bóc tách comment (SKU, Màu, Size, Ý định mua hàng)</p>
        </div>
      </header>

      <div class="debug-grid">
        <!-- Input section -->
        <div class="input-section">
          <h3>Nhập Comment (Mô phỏng thực tế)</h3>
          <textarea 
            v-model="commentInput" 
            class="input-field textarea" 
            placeholder="VD: chốt cho em mã A12 size m màu đỏ nhé shop ơi x2"
            rows="4"
          ></textarea>
          
          <button class="btn-primary mt-4" @click="handleDebug" :disabled="loading">
            <span v-if="loading">Đang phân tích...</span>
            <span v-else>Phân tích Comment</span>
          </button>
        </div>

        <!-- Output Result Section -->
        <div class="result-section" v-if="result">
          <h3>Kết Quả Parser</h3>
          
          <div class="result-card">
            <div class="result-row">
              <span class="label">Ý định (Intent):</span>
              <span class="value badge" :class="getIntentClass(result.intent)">{{ result.intent }}</span>
            </div>
            
            <div class="result-row">
              <span class="label">Mã SKU:</span>
              <span class="value highlight-sku">{{ result.sku || 'Không tìm thấy' }}</span>
            </div>

            <div class="result-row">
              <span class="label">Số lượng:</span>
              <span class="value">{{ result.quantity }}</span>
            </div>

            <div class="result-row">
              <span class="label">Phân loại (Variants):</span>
              <div class="variant-tags">
                <span v-for="v in result.variants" :key="v" class="variant-tag">{{ v }}</span>
                <span v-if="result.variants.length === 0" class="muted">Không có</span>
              </div>
            </div>

            <div class="result-row">
              <span class="label">Độ tin cậy (Confidence):</span>
              <div class="confidence-bar-container">
                <div class="confidence-bar" :style="{ width: `${result.confidenceScore * 100}%`, background: getConfidenceColor(result.confidenceScore) }"></div>
                <span>{{ (result.confidenceScore * 100).toFixed(0) }}%</span>
              </div>
            </div>

            <div class="result-row">
              <span class="label">Trạng thái:</span>
              <span v-if="result.needsReview" class="value text-warning">Cần nhân viên xem lại (Review)</span>
              <span v-else class="value text-success">Chắc chắn (Tự động chốt)</span>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Developer Trace -->
      <div class="trace-section" v-if="result">
        <h3>Log Kỹ Thuật (Trace)</h3>
        <pre class="trace-box">Normalized Text: "{{ result.normalizedText }}"
Matched Rules:
{{ result.matchedRules.map(r => `- ${r}`).join('\n') || '- None' }}</pre>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { ParsedIntent } from '../types/parser';

const commentInput = ref('');
const result = ref<ParsedIntent | null>(null);
const loading = ref(false);

const handleDebug = async () => {
  if (!commentInput.value.trim()) return;
  loading.value = true;
  try {
    result.value = await invoke<ParsedIntent>('debug_parse_comment', { comment: commentInput.value });
  } catch (e) {
    console.error("Lỗi parse:", e);
    alert("Có lỗi khi phân tích: " + e);
  } finally {
    loading.value = false;
  }
};

const getIntentClass = (intent: string) => {
  switch (intent) {
    case 'Buy': return 'badge-success';
    case 'Cancel': return 'badge-danger';
    case 'Question': return 'badge-info';
    default: return 'badge-muted';
  }
};

const getConfidenceColor = (score: number) => {
  if (score >= 0.8) return '#22c55e'; // green
  if (score >= 0.5) return '#f59e0b'; // orange
  return '#ef4444'; // red
};
</script>

<style scoped>
.parser-debug {
  padding: 1rem 2rem 2rem 2rem;
  display: flex;
  height: 100%;
  box-sizing: border-box;
}

.glass-panel {
  background: var(--surface);
  backdrop-filter: blur(16px);
  border: 1px solid var(--border);
  border-radius: 24px;
  width: 100%;
  padding: 2.5rem;
  display: flex;
  flex-direction: column;
}

.header-text h1 {
  margin: 0;
  font-size: 2.2rem;
  background: linear-gradient(135deg, #4ade80, #3b82f6);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.header-text p {
  color: var(--text-muted);
  margin: 0.5rem 0 2rem 0;
}

.debug-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2rem;
}

h3 {
  font-size: 1.1rem;
  margin-top: 0;
  margin-bottom: 1rem;
  color: #e2e8f0;
}

.input-field {
  width: 100%;
  background: rgba(15, 23, 42, 0.6);
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 1rem;
  color: var(--text-main);
  font-size: 1rem;
  font-family: inherit;
  resize: vertical;
}

.input-field:focus {
  outline: none;
  border-color: var(--primary);
}

.btn-primary {
  background: var(--primary);
  color: white;
  border: none;
  padding: 0.8rem 1.5rem;
  border-radius: 12px;
  font-weight: 600;
  cursor: pointer;
  width: 100%;
}

.mt-4 {
  margin-top: 1rem;
}

.result-card {
  background: rgba(15, 23, 42, 0.4);
  border: 1px solid var(--border);
  border-radius: 16px;
  padding: 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.result-row {
  display: flex;
  align-items: flex-start;
  gap: 1rem;
}

.label {
  width: 140px;
  color: var(--text-muted);
  font-weight: 500;
  flex-shrink: 0;
}

.value {
  font-weight: 600;
  color: var(--text-main);
}

.highlight-sku {
  color: #c084fc;
  font-family: monospace;
  font-size: 1.1rem;
  background: rgba(192, 132, 252, 0.1);
  padding: 0.2rem 0.5rem;
  border-radius: 6px;
}

.badge {
  padding: 0.3rem 0.8rem;
  border-radius: 99px;
  font-size: 0.9rem;
}
.badge-success { background: rgba(34, 197, 94, 0.2); color: #4ade80; }
.badge-danger { background: rgba(239, 68, 68, 0.2); color: #f87171; }
.badge-info { background: rgba(59, 130, 246, 0.2); color: #60a5fa; }
.badge-muted { background: rgba(148, 163, 184, 0.2); color: #cbd5e1; }

.variant-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}
.variant-tag {
  background: rgba(255, 255, 255, 0.1);
  padding: 0.2rem 0.6rem;
  border-radius: 6px;
  font-size: 0.85rem;
}
.muted {
  color: var(--text-muted);
  font-weight: normal;
}

.confidence-bar-container {
  display: flex;
  align-items: center;
  gap: 1rem;
  flex: 1;
}

.confidence-bar {
  height: 8px;
  border-radius: 99px;
  transition: width 0.3s ease;
}

.text-warning { color: #f59e0b; }
.text-success { color: #4ade80; }

.trace-section {
  margin-top: 2rem;
  border-top: 1px solid var(--border);
  padding-top: 1.5rem;
}

.trace-box {
  background: #0f172a;
  padding: 1rem;
  border-radius: 8px;
  font-family: monospace;
  color: #a78bfa;
  font-size: 0.9rem;
  white-space: pre-wrap;
}
</style>
