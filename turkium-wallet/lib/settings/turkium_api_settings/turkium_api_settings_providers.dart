import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/core_providers.dart';
import '../settings_providers.dart';
import 'turkium_api_settings_notifier.dart';
import 'turkium_api_settings_types.dart';

final turkiumApiSettingsProvider =
    StateNotifierProvider<TurkiumApiSettingsNotifier, TurkiumApiSettings>((ref) {
  final repository = ref.watch(settingsRepositoryProvider);
  return TurkiumApiSettingsNotifier(repository);
});

final turkiumApiDefaultUrlProvider = Provider.autoDispose((ref) {
  final networkId = ref.watch(networkIdProvider);

  final apiUrl = TurkiumApiSettings.defaultApiUrlForNetworkId(networkId);

  return apiUrl;
});

final turkiumApiUrlProvider = Provider.autoDispose((ref) {
  final settings = ref.watch(turkiumApiSettingsProvider);
  final networkId = ref.watch(networkIdProvider);

  final apiUrl = settings.apiUrlForNetworkId(networkId);

  return apiUrl;
});

final turkiumApiUserSetUrlProvider = Provider.autoDispose((ref) {
  final defaultApiUrl = ref.watch(turkiumApiDefaultUrlProvider);
  final apiUrl = ref.watch(turkiumApiUrlProvider);

  return apiUrl == defaultApiUrl ? '' : apiUrl;
});
