import 'package:freezed_annotation/freezed_annotation.dart';

import '../../turkium/network.dart';

part 'turkium_api_settings_types.freezed.dart';
part 'turkium_api_settings_types.g.dart';

const kTurkiumApiUrlMainnet = 'https://api.turkium.org';
const kTurkiumApiUrlTestnet10 = 'https://api-tn10.turkium.org';
const kTurkiumApiUrlTestnet11 = 'https://api-tn11.turkium.org';

@freezed
sealed class TurkiumApiSettings with _$TurkiumApiSettings {
  const TurkiumApiSettings._();

  const factory TurkiumApiSettings({
    @Default({}) Map<String, String> apiUrlByNetworkId,
  }) = _TurkiumApiSettings;

  factory TurkiumApiSettings.fromJson(Map<String, dynamic> json) =>
      _$TurkiumApiSettingsFromJson(json);

  static String defaultApiUrlForNetworkId(String networkId) {
    return switch (networkId) {
      kTurkiumNetworkIdMainnet => kTurkiumApiUrlMainnet,
      kTurkiumNetworkIdTestnet10 => kTurkiumApiUrlTestnet10,
      kTurkiumNetworkIdTestnet11 => kTurkiumApiUrlTestnet11,
      _ => '',
    };
  }

  String apiUrlForNetworkId(String networkId) {
    return apiUrlByNetworkId[networkId] ?? defaultApiUrlForNetworkId(networkId);
  }
}
