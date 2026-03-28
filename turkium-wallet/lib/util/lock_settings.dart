import 'vault.dart';

class LockSettings {
  static const _turkium_lock = 'fturkium_lock_dev';
  static const _turkium_autolock = 'fturkium_autolock_dev';

  static const _true = 'true';
  static const _false = 'false';

  final Vault vault;
  LockSettings(this.vault);

  Future<void> setLock(bool value) =>
      vault.set(_turkium_lock, value ? _true : _false);

  Future<bool> getLock() async =>
      (await vault.get(_turkium_lock) ?? _true) == _true;

  Future<void> setAutoLock(bool value) =>
      vault.set(_turkium_autolock, value ? _true : _false);

  Future<bool> getAutoLock() async =>
      (await vault.get(_turkium_autolock) ?? _true) == _true;
}
