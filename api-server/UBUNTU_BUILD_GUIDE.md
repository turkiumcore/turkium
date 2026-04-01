# Ubuntu'da Turkium API Server Binary Derleme Rehberi

## 🎯 Seçenekler

### Option 1: Ubuntu'da Doğrudan Derleme (Önerilen)

Ubuntu sunucusunda doğrudan derlemek en güvenilir yöntemdir.

#### Adım 1: Rust Kurulumu

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
```

#### Adım 2: Kaynak Kodunu Kopyala

```bash
# macOS'tan Ubuntu'ya kopyala
scp -r Turkium/api-server user@ubuntu-server:/tmp/

# Veya git clone
git clone https://github.com/turkium/turkium.git
cd Turkium/api-server
```

#### Adım 3: Binary Derle

```bash
cd /tmp/api-server
cargo build --release
```

**Beklenen Çıktı:**
```
✅ Compilation: SUCCESS
✅ Binary: target/release/turkium-api-server
✅ Size: ~4-5MB (Linux binary)
```

#### Adım 4: Systemd'ye Kur

```bash
sudo bash install-systemd.sh
```

---

### Option 2: macOS'tan Cross-Compile

macOS'ta Ubuntu için binary derlemek için cross-compile kullanabilirsiniz.

#### Adım 1: Cross Kurulumu

```bash
cargo install cross
```

#### Adım 2: Linux Binary Derle

```bash
cd Turkium/api-server
cross build --release --target x86_64-unknown-linux-gnu
```

**Beklenen Çıktı:**
```
✅ Binary: target/x86_64-unknown-linux-gnu/release/turkium-api-server
✅ Size: ~4-5MB (Linux binary)
```

#### Adım 3: Ubuntu'ya Kopyala

```bash
scp target/x86_64-unknown-linux-gnu/release/turkium-api-server user@ubuntu-server:/tmp/
```

#### Adım 4: Ubuntu'da Systemd'ye Kur

```bash
ssh user@ubuntu-server
cd /tmp
sudo bash install-systemd.sh
```

---

### Option 3: Docker ile Derleme

Docker kullanarak Ubuntu binary'si derleyebilirsiniz.

#### Adım 1: Docker Image Oluştur

```bash
cd Turkium/api-server
docker build -t turkium-api-builder:latest .
```

#### Adım 2: Container'da Derle

```bash
docker run --rm -v $(pwd):/app turkium-api-builder:latest \
  cargo build --release
```

#### Adım 3: Binary'yi Çıkar

```bash
# Binary şurada olacak:
ls -lh target/release/turkium-api-server
```

#### Adım 4: Ubuntu'ya Kopyala

```bash
scp target/release/turkium-api-server user@ubuntu-server:/tmp/
```

---

## 🚀 Hızlı Kurulum (Ubuntu'da)

### 1. Rust Kurulumu

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Kaynak Kodunu Kopyala

```bash
git clone https://github.com/turkium/turkium.git
cd Turkium/api-server
```

### 3. Binary Derle

```bash
cargo build --release
```

### 4. Systemd'ye Kur

```bash
sudo bash install-systemd.sh
```

### 5. Konfigürasyon Düzenle

```bash
sudo nano /etc/turkium-api-server/.env
```

### 6. Service'i Başlat

```bash
sudo systemctl start turkium-api
```

### 7. Test Et

```bash
curl http://localhost:3001/health
```

---

## 📊 Build Zamanları

| Yöntem | Zaman | Yer |
|--------|-------|-----|
| **Ubuntu'da Doğrudan** | 1-2 dakika | Ubuntu sunucusu |
| **Cross-Compile** | 2-3 dakika | macOS |
| **Docker** | 3-5 dakika | macOS/Linux |

---

## 🔍 Binary Kontrol

### Ubuntu'da Derlenmiş Binary

```bash
file target/release/turkium-api-server
# Beklenen: ELF 64-bit LSB shared object, x86-64
```

### macOS'ta Derlenmiş Binary

```bash
file target/release/turkium-api-server
# Beklenen: Mach-O 64-bit executable arm64
```

---

## ⚠️ Önemli Notlar

1. **Binary Uyumluluğu**: Ubuntu'da derlenmiş binary sadece Linux'ta çalışır
2. **Rust Versiyonu**: Rust 1.70+ gereklidir
3. **Dependencies**: Derleme sırasında internet bağlantısı gereklidir
4. **Disk Alanı**: Derleme için ~2GB disk alanı gereklidir

---

## 🆘 Troubleshooting

### "cargo: command not found"

```bash
# Rust kurulumu kontrol et
rustc --version
cargo --version

# Eğer kurulu değilse:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### "error: linker `cc` not found"

```bash
# Ubuntu'da build tools kurulumu
sudo apt-get update
sudo apt-get install -y build-essential
```

### "error: could not compile `turkium-api-server`"

```bash
# Cargo cache'i temizle
cargo clean

# Tekrar derle
cargo build --release
```

### Binary çalışmıyor

```bash
# Binary'nin çalıştırılabilir olup olmadığını kontrol et
chmod +x target/release/turkium-api-server

# Doğrudan çalıştır
./target/release/turkium-api-server
```

---

## 📋 Deployment Checklist

- [ ] Rust 1.70+ kurulu
- [ ] Binary derlenmiş
- [ ] Binary Ubuntu'da çalışıyor
- [ ] Systemd kurulumu tamamlandı
- [ ] Konfigürasyon dosyası düzenlenmiş
- [ ] Service başlatılmış
- [ ] Health check çalışıyor
- [ ] Logs kontrol edilmiş

---

## 🎯 Önerilen Yöntem

**Ubuntu'da Doğrudan Derleme** en güvenilir ve en hızlı yöntemdir:

```bash
# Ubuntu sunucusunda
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

git clone https://github.com/turkium/turkium.git
cd Turkium/api-server

cargo build --release
sudo bash install-systemd.sh
```

---

**Turkium API Server - Ubuntu Binary Derleme Rehberi Tamamlandı! 🚀**
