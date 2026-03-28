import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../settings_repository.dart';
import 'turkium_api_settings_types.dart';

const kTurkiumApiSettingsKey = '_kTurkiumApiSettingsKey';

extension TurkiumApiSettingsExtension on SettingsRepository {
  TurkiumApiSettings getTurkiumApiSettings() {
    return box.tryGet<TurkiumApiSettings>(kTurkiumApiSettingsKey,
            typeFactory: TurkiumApiSettings.fromJson) ??
        const TurkiumApiSettings();
  }

  Future<void> setTurkiumApiSettings(TurkiumApiSettings settings) {
    return box.set(kTurkiumApiSettingsKey, settings);
  }
}

class TurkiumApiSettingsNotifier extends StateNotifier<TurkiumApiSettings> {
  final SettingsRepository repository;
  TurkiumApiSettingsNotifier(this.repository)
      : super(repository.getTurkiumApiSettings());

  Future<void> setApiUrl(
    String apiUrl, {
    required String networkId,
  }) async {
    if (state.apiUrlForNetworkId(networkId) == apiUrl) {
      return;
    }

    final apiUrlByNetworkId = {
      ...state.apiUrlByNetworkId,
      networkId: apiUrl,
    };

    if (apiUrl.isEmpty) {
      apiUrlByNetworkId.remove(networkId);
    }

    final settings = state.copyWith(apiUrlByNetworkId: apiUrlByNetworkId);

    await repository.setTurkiumApiSettings(settings);
    state = settings;
  }
}
