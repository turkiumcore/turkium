// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'turkium_api_settings_types.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_TurkiumApiSettings _$TurkiumApiSettingsFromJson(Map json) =>
    _TurkiumApiSettings(
      apiUrlByNetworkId:
          (json['apiUrlByNetworkId'] as Map?)?.map(
            (k, e) => MapEntry(k as String, e as String),
          ) ??
          const {},
    );

Map<String, dynamic> _$TurkiumApiSettingsToJson(_TurkiumApiSettings instance) =>
    <String, dynamic>{'apiUrlByNetworkId': instance.apiUrlByNetworkId};
