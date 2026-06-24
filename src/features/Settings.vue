<template>
  <div class="settings-page">
    <div class="glass-panel main-content">
      <header class="header-section">
        <div class="header-text">
          <h1>Cài Đặt Hệ Thống</h1>
          <p>Tùy chỉnh thông số ứng dụng và các công cụ hỗ trợ</p>
        </div>
      </header>

      <div class="settings-grid">
        <!-- System Settings -->
        <div class="settings-card">
          <div class="card-header">
            <h3>⚙️ Cài Đặt Chung</h3>
          </div>
          <div class="card-body">
            <p class="text-muted">Tính năng đang được phát triển...</p>
          </div>
        </div>

        <!-- Advanced Tools -->
        <div class="settings-card">
          <div class="card-header">
            <h3>🛠️ Công Cụ Nâng Cao (Dành cho Dev)</h3>
          </div>
          <div class="card-body">
            <div class="tool-item">
              <div class="tool-info">
                <h4>Kiểm Tra Cú Pháp (Parser Debug)</h4>
                <p>Công cụ gỡ lỗi thuật toán bóc tách comment chốt đơn.</p>
              </div>
              <button class="btn-secondary" @click="$router.push('/parser-debug')">Mở Công Cụ</button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// Removed unused router import as we use $router directly in template
</script>

<style scoped>
.settings-page {
  padding: 1rem 2rem 2rem 2rem;
  display: flex;
  justify-content: center;
  height: 100%;
  box-sizing: border-box;
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

.header-text h1 {
  margin: 0;
  font-size: 2.2rem;
  background: linear-gradient(135deg, #a78bfa, #f472b6);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.header-text p {
  color: var(--text-muted);
  margin: 0.5rem 0 2rem 0;
}

.settings-grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: 1.5rem;
}

.settings-card {
  background: rgba(15, 23, 42, 0.4);
  border: 1px solid var(--border);
  border-radius: 16px;
  overflow: hidden;
}

.card-header {
  padding: 1rem 1.5rem;
  background: rgba(255, 255, 255, 0.03);
  border-bottom: 1px solid var(--border);
}

.card-header h3 {
  margin: 0;
  font-size: 1.1rem;
  color: #e2e8f0;
}

.card-body {
  padding: 1.5rem;
}

.tool-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: rgba(15, 23, 42, 0.6);
  border: 1px solid var(--border);
  padding: 1rem;
  border-radius: 12px;
}

.tool-info h4 {
  margin: 0 0 0.2rem 0;
  color: var(--text-main);
  font-size: 1rem;
}

.tool-info p {
  margin: 0;
  color: var(--text-muted);
  font-size: 0.9rem;
}

.text-muted {
  color: var(--text-muted);
  font-style: italic;
}

.btn-secondary {
  background: rgba(255, 255, 255, 0.1);
  color: white;
  border: 1px solid var(--border);
  padding: 0.6rem 1.2rem;
  border-radius: 8px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary:hover {
  background: rgba(255, 255, 255, 0.2);
}
</style>
