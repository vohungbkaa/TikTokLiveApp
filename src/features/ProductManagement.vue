<template>
  <div class="product-management">
    <div class="glass-panel main-content">
      <header class="header-section">
        <div class="header-text">
          <h1>Quản Lý Kho Hàng</h1>
          <p>Tạo mới và theo dõi tồn kho sản phẩm</p>
        </div>
        <div class="header-actions">
          <button class="btn-secondary">Import CSV</button>
          <button class="btn-primary" @click="showCreateModal = true">Thêm Sản Phẩm Mới</button>
        </div>
      </header>

      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-title">Tổng Sản Phẩm</div>
          <div class="stat-value">{{ products.length }}</div>
        </div>
        <div class="stat-card warning">
          <div class="stat-title">Sắp Hết Hàng</div>
          <div class="stat-value">{{ lowStockCount }}</div>
        </div>
        <div class="stat-card danger">
          <div class="stat-title">Hết Hàng</div>
          <div class="stat-value">{{ outOfStockCount }}</div>
        </div>
      </div>

      <div class="table-container">
        <table class="modern-table">
          <thead>
            <tr>
              <th>SKU</th>
              <th>Tên Sản Phẩm</th>
              <th>Giá Bán</th>
              <th>Tồn Kho</th>
              <th>Chế Độ Bán</th>
              <th>Trạng Thái</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="product in products" :key="product.id">
              <td class="sku-cell">{{ product.sku }}</td>
              <td>{{ product.name }}</td>
              <td class="price-cell">{{ formatPrice(product.price) }}</td>
              <td>
                <span class="stock-badge" :class="getStockClass(product.stockQty)">
                  {{ product.stockQty }}
                </span>
              </td>
              <td>
                <span class="mode-badge">{{ product.sellingMode }}</span>
              </td>
              <td>
                <div class="status-wrapper">
                  <span class="status-dot" :class="{ active: product.isActive }"></span>
                  {{ product.isActive ? 'Đang bán' : 'Ngừng bán' }}
                </div>
              </td>
            </tr>
            <tr v-if="products.length === 0">
              <td colspan="6" class="empty-state">Chưa có sản phẩm nào. Nhấn "Thêm Sản Phẩm Mới" để bắt đầu.</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Create Modal -->
    <div v-if="showCreateModal" class="modal-overlay" @click.self="showCreateModal = false">
      <div class="modal-content glass-panel">
        <h2>Thêm Sản Phẩm Mới</h2>
        <form @submit.prevent="handleCreateProduct" class="modern-form">
          <div class="form-group">
            <label>SKU (Mã SP)</label>
            <input v-model="form.sku" type="text" required class="input-field" placeholder="VD: A12" />
          </div>
          <div class="form-group">
            <label>Tên Sản Phẩm</label>
            <input v-model="form.name" type="text" required class="input-field" placeholder="VD: Áo Thun Cổ Tròn" />
          </div>
          <div class="form-grid">
            <div class="form-group">
              <label>Giá Bán (VND)</label>
              <input v-model.number="form.price" type="number" required class="input-field" />
            </div>
            <div class="form-group">
              <label>Tồn Kho</label>
              <input v-model.number="form.stockQty" type="number" required class="input-field" />
            </div>
          </div>
          <div class="form-group">
            <label>Chế Độ Bán</label>
            <select v-model="form.sellingMode" class="input-field custom-select-native">
              <option value="stock">Trừ Tồn Kho (Có sẵn số lượng)</option>
              <option value="unique">Hàng Độc Bản (1 cái duy nhất)</option>
              <option value="preorder">Pre-order (Gom đơn không giới hạn)</option>
            </select>
          </div>
          <div class="modal-actions">
            <button type="button" class="btn-ghost" @click="showCreateModal = false">Hủy</button>
            <button type="submit" class="btn-primary" :disabled="loading">
              <span v-if="loading" class="spinner"></span>
              <span v-else>Lưu Sản Phẩm</span>
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Product, CreateProductInput } from '../types/product';

const products = ref<Product[]>([]);
const showCreateModal = ref(false);
const loading = ref(false);

const form = ref<CreateProductInput>({
  sku: '',
  name: '',
  price: 0,
  sellingMode: 'stock',
  stockQty: 0
});

const lowStockCount = computed(() => products.value.filter(p => p.stockQty > 0 && p.stockQty <= 5).length);
const outOfStockCount = computed(() => products.value.filter(p => p.stockQty <= 0).length);

const fetchProducts = async () => {
  try {
    products.value = await invoke<Product[]>('list_products');
  } catch (e) {
    console.error("Lỗi tải danh sách sản phẩm:", e);
  }
};

const handleCreateProduct = async () => {
  loading.value = true;
  try {
    const newProduct = await invoke<Product>('create_product', { input: form.value });
    products.value.unshift(newProduct);
    showCreateModal.value = false;
    form.value = { sku: '', name: '', price: 0, sellingMode: 'stock', stockQty: 0 };
  } catch (e) {
    console.error("Lỗi tạo sản phẩm:", e);
    alert("Không thể tạo sản phẩm. Mã SKU có thể bị trùng hoặc có lỗi cơ sở dữ liệu.");
  } finally {
    loading.value = false;
  }
};

const formatPrice = (price: number) => {
  return new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(price);
};

const getStockClass = (qty: number) => {
  if (qty <= 0) return 'danger';
  if (qty <= 5) return 'warning';
  return 'safe';
};

onMounted(() => {
  fetchProducts();
});
</script>

<style scoped>
.product-management {
  display: flex;
  padding: 1rem 2rem 2rem 2rem;
  height: 100%;
  box-sizing: border-box;
  overflow: hidden;
}

.glass-panel {
  background: var(--surface);
  backdrop-filter: blur(16px);
  -webkit-backdrop-filter: blur(16px);
  border: 1px solid var(--border);
  border-radius: 24px;
  box-shadow: var(--glass-shadow);
  width: 100%;
  padding: 2.5rem;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.header-section {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
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
  margin: 0.5rem 0 0 0;
}

.header-actions {
  display: flex;
  gap: 1rem;
}

.btn-secondary {
  background: transparent;
  color: var(--text-main);
  border: 1px solid var(--border);
  padding: 0.8rem 1.2rem;
  border-radius: 12px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary:hover {
  background: rgba(255, 255, 255, 0.05);
  border-color: var(--text-muted);
}

.btn-primary {
  background: var(--primary);
  color: white;
  border: none;
  padding: 0.8rem 1.5rem;
  border-radius: 12px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  box-shadow: 0 4px 14px 0 rgba(59, 130, 246, 0.39);
}

.btn-primary:hover {
  background: var(--primary-hover);
  transform: translateY(-2px);
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1.5rem;
  margin-bottom: 2.5rem;
}

.stat-card {
  background: rgba(15, 23, 42, 0.4);
  border: 1px solid var(--border);
  border-radius: 16px;
  padding: 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  transition: transform 0.2s;
}

.stat-card:hover {
  transform: translateY(-4px);
}

.stat-title {
  color: var(--text-muted);
  font-size: 0.9rem;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.stat-value {
  font-size: 2.5rem;
  font-weight: 700;
  color: var(--text-main);
}

.stat-card.warning .stat-value { color: #f59e0b; }
.stat-card.danger .stat-value { color: #ef4444; }

.table-container {
  overflow-y: auto;
  overflow-x: auto;
  border-radius: 12px;
  border: 1px solid var(--border);
  flex: 1;
}

.modern-table {
  width: 100%;
  border-collapse: collapse;
  text-align: left;
}

.modern-table th {
  background: rgba(15, 23, 42, 0.8);
  padding: 1.2rem 1rem;
  color: var(--text-muted);
  font-weight: 600;
  font-size: 0.9rem;
  border-bottom: 1px solid var(--border);
}

.modern-table td {
  padding: 1.2rem 1rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  color: var(--text-main);
}

.modern-table tbody tr:hover {
  background: rgba(255, 255, 255, 0.02);
}

.sku-cell {
  font-weight: 600;
  font-family: monospace;
  font-size: 1.1rem;
  color: #c084fc;
}

.price-cell {
  font-weight: 500;
  color: #4ade80;
}

.stock-badge {
  padding: 0.3rem 0.8rem;
  border-radius: 99px;
  font-weight: 700;
  font-size: 0.9rem;
}

.stock-badge.safe { background: rgba(74, 222, 128, 0.15); color: #4ade80; }
.stock-badge.warning { background: rgba(245, 158, 11, 0.15); color: #f59e0b; }
.stock-badge.danger { background: rgba(239, 68, 68, 0.15); color: #ef4444; }

.mode-badge {
  background: rgba(148, 163, 184, 0.15);
  color: #cbd5e1;
  padding: 0.3rem 0.8rem;
  border-radius: 8px;
  font-size: 0.85rem;
}

.status-wrapper {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #ef4444;
}

.status-dot.active {
  background: #22c55e;
  box-shadow: 0 0 8px rgba(34, 197, 94, 0.5);
}

.empty-state {
  text-align: center;
  padding: 3rem !important;
  color: var(--text-muted);
  font-style: italic;
}

/* Modal Styles */
.modal-overlay {
  position: fixed;
  top: 0; left: 0; right: 0; bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(4px);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 100;
}

.modal-content {
  width: 100%;
  max-width: 500px;
  padding: 2.5rem;
}

.modal-content h2 {
  margin-top: 0;
  margin-bottom: 1.5rem;
}

.form-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.modern-form {
  display: flex;
  flex-direction: column;
  gap: 1.2rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-group label {
  font-size: 0.9rem;
  color: var(--text-muted);
}

.input-field {
  background: rgba(15, 23, 42, 0.6);
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: 0.8rem 1rem;
  color: var(--text-main);
  font-size: 1rem;
  transition: all 0.2s;
}

.input-field:focus {
  outline: none;
  border-color: var(--primary);
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2);
}

.custom-select-native {
  appearance: none;
  background-image: url("data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='%2394a3b8' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3e%3cpolyline points='6 9 12 15 18 9'%3e%3c/polyline%3e%3c/svg%3e");
  background-repeat: no-repeat;
  background-position: right 1rem center;
  background-size: 1em;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  margin-top: 1.5rem;
}

.btn-ghost {
  background: transparent;
  color: var(--text-muted);
  border: none;
  padding: 0.8rem 1.5rem;
  border-radius: 10px;
  cursor: pointer;
}

.btn-ghost:hover {
  background: rgba(255, 255, 255, 0.05);
  color: var(--text-main);
}
</style>
