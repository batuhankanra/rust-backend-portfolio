# Rust Portfolio Backend API 🦀

Bu proje, modern web standartlarına uygun, güvenli ve ölçeklenebilir bir **REST API** altyapısıdır. Axum framework'ü ve MongoDB kullanılarak geliştirilmiştir.

## 🚀 Öne Çıkan Özellikler

* **Asenkron Çalışma:** `Tokio` runtime ve `Axum` framework'ü ile yüksek performans.
* **JWT Authentication:** JSON Web Token tabanlı kimlik doğrulama.
* **RBAC (Role-Based Access Control):** Admin ve User rollerine göre gelişmiş yetkilendirme sistemi.
* **MongoDB Entegrasyonu:** `mongodb-rust-driver` ile doküman tabanlı veri yönetimi.
* **Merkezi Hata Yönetimi:** `thiserror` ve `IntoResponse` kullanılarak özelleştirilmiş JSON hata çıktıları.
* **Güvenli Şifreleme:** `bcrypt` ile şifrelerin güvenli şekilde hash'lenmesi.

## 🛠️ Teknoloji Yığını

* **Dil:** Rust 🦀
* **Framework:** [Axum](https://github.com/tokio-rs/axum)
* **Veritabanı:** MongoDB
* **Kriptografi:** `jsonwebtoken`, `bcrypt`
* **Serileştirme:** `serde`, `serde_json`

## 📁 Proje Yapısı

```text
src/
├── auth/           # JWT oluşturma ve doğrulama mantığı
├── config/         # Çevresel değişkenler ve yapılandırma
├── database/       # MongoDB bağlantı ayarları
├── handlers/       # HTTP isteklerini karşılayan fonksiyonlar
├── middleware/     # Auth Guard ve yetkilendirme katmanları
├── models/         # Veritabanı şemaları (User, Project)
├── routes/         # Rota tanımlamaları
└── error.rs        # Merkezi hata (AppError) tanımları