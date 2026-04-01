# turkium API Server - Quick Start Guide

## 🚀 5 Dakikada Başlayın

### 1️⃣ Binary Derle (2 dakika)

```bash
cd Turkium/api-server
cargo build --release
```

### 2️⃣ Systemd'ye Kur (1 dakika)

```bash
sudo bash install-systemd.sh
```

### 3️⃣ Konfigürasyon Düzenle (1 dakika)

```bash
sudo nano /etc/turkium-api-server/.env
```

### 4️⃣ Service'i Başlat (30 saniye)

```bash
sudo systemctl start turkium-api
```

### 5️⃣ Test Et (30 saniye)

```bash
curl http://localhost:3001/health
```

---

## 📋 Sık Kullanılan Komutlar

### Service Yönetimi

```bash
# Başlat
sudo systemctl start turkium-api

# Durdur
sudo systemctl stop turkium-api

# Yeniden başlat
sudo systemctl restart turkium-api

# Status
sudo systemctl status turkium-api

# Enable (boot'ta başlasın)
sudo systemctl enable turkium-api

# Disable (boot'ta başlamasın)
sudo systemctl disable turkium-api
```

### Logs

```bash
# Live logs
sudo journalctl -u turkium-api -f

# Son 50 satır
sudo journalctl -u turkium-api -n 50

# Son 1 saat
sudo journalctl -u turkium-api --since "1 hour ago"
```

### Management Script

```bash
# Başlat
sudo ./manage-service.sh start

# Durdur
sudo ./manage-service.sh stop

# Yeniden başlat
sudo ./manage-service.sh restart

# Status
sudo ./manage-service.sh status

# Logs
sudo ./manage-service.sh logs

# Health check
./manage-service.sh health

# Konfigürasyon düzenle
sudo ./manage-service.sh config
```

---

## 🔍 Troubleshooting

### Service Başlamıyor?

```bash
# Logs kontrol et
sudo journalctl -u turkium-api -n 50

# Status kontrol et
sudo systemctl status turkium-api

# Node bağlantısı kontrol et
curl http://localhost:5200
```

### Port Zaten Kullanımda?

```bash
# Port'u kullanan process'i bul
sudo lsof -i :3001

# Process'i öldür
sudo kill -9 <PID>

# Service'i yeniden başlat
sudo systemctl restart turkium-api
```

### High Memory Usage?

```bash
# Cache ayarlarını azalt
sudo nano /etc/turkium-api-server/.env

# Şu satırları değiştir:
# CACHE_MAX_CAPACITY=5000
# CACHE_TTL_SECS=15

# Service'i yeniden başlat
sudo systemctl restart turkium-api
```

---

## 📁 Dosya Konumları

| Dosya | Konum |
|-------|-------|
| Binary | `/opt/turkium-api-server/turkium-api-server` |
| Config | `/etc/turkium-api-server/.env` |
| Logs | `journalctl -u turkium-api` |
| Service | `/etc/systemd/system/turkium-api.service` |

---

## ✅ Health Check

```bash
# API health
curl http://localhost:3001/health

# Beklenen yanıt:
# {
#   "status": "ok",
#   "timestamp": "2024-03-31T10:30:45.123Z",
#   "node_connected": true
# }
```

---

## 🔗 API Endpoints

```bash
# BlockDAG info
curl http://localhost:3001/info/blockdag

# Coin supply
curl http://localhost:3001/info/coinsupply

# Hashrate
curl http://localhost:3001/info/hashrate

# Address balance
curl http://localhost:3001/addresses/turkium:qzq34kp6kxqrput2n60sf9qdaealtpwlrd4nhnets479kd3nwd3t6xgv9xd45/balance
```

---

## 📊 Performance

| Metrik | Değer |
|--------|-------|
| Requests/sec | 50,000+ |
| Latency | 1-5ms |
| Memory | 5-10MB |
| Binary Size | 4.1MB |

---

## 🆘 Yardım

```bash
# Management script yardımı
./manage-service.sh help

# Systemctl yardımı
man systemctl

# Journalctl yardımı
man journalctl
```

---

**Turkium API Server - Hazır! 🚀**
