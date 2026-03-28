import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../settings_providers.dart';
import 'node_settings_notifier.dart';
import 'node_types.dart';

final turkiumNodeSettingsProvider =
    StateNotifierProvider<NodeSettingsNotifier, NodeConfigSettings>((ref) {
  final repository = ref.watch(settingsRepositoryProvider);
  final notifier = NodeSettingsNotifier(repository);
  return notifier;
});

final turkiumNodeOptionsProvider = Provider((ref) {
  final settings = ref.watch(turkiumNodeSettingsProvider);
  return settings.options;
});

final turkiumNodeConfigProvider = Provider((ref) {
  final settings = ref.watch(turkiumNodeSettingsProvider);
  return ActiveNodeConfig(config: settings.selected);
});
