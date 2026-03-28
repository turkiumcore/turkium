// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'block_explorers.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_BlockExplorer _$BlockExplorerFromJson(Map json) => _BlockExplorer(
  network: $enumDecode(_$TurkiumNetworkEnumMap, json['network']),
  name: json['name'] as String,
  url: json['url'] as String,
  addressUrl: json['addressUrl'] as String,
  txUrl: json['txUrl'] as String,
);

Map<String, dynamic> _$BlockExplorerToJson(_BlockExplorer instance) =>
    <String, dynamic>{
      'network': _$TurkiumNetworkEnumMap[instance.network]!,
      'name': instance.name,
      'url': instance.url,
      'addressUrl': instance.addressUrl,
      'txUrl': instance.txUrl,
    };

const _$TurkiumNetworkEnumMap = {
  TurkiumNetwork.mainnet: 'mainnet',
  TurkiumNetwork.testnet: 'testnet',
  TurkiumNetwork.devnet: 'devnet',
  TurkiumNetwork.simnet: 'simnet',
};
