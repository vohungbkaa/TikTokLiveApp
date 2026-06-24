export interface Product {
  id: string;
  sku: string;
  name: string;
  price: number;
  sellingMode: string;
  stockQty: number;
  isActive: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface CreateProductInput {
  sku: string;
  name: string;
  price: number;
  sellingMode: string;
  stockQty: number;
}
