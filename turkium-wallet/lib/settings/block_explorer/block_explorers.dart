import 'package:freezed_annotation/freezed_annotation.dart';

import '../../turkium/turkium.dart';

part 'block_explorers.freezed.dart';
part 'block_explorers.g.dart';

const kParamPattern = '{0}';

@freezed
sealed class BlockExplorer with _$BlockExplorer {
  const BlockExplorer._();
  const factory BlockExplorer({
    required TurkiumNetwork network,
    required String name,
    required String url,
    required String addressUrl,
    required String txUrl,
  }) = _BlockExplorer;

  String urlForAddress(String address) =>
      addressUrl.replaceFirst(kParamPattern, address);

  String urlForTx(String hash) => txUrl.replaceFirst(kParamPattern, hash);

  factory BlockExplorer.fromJson(Map<String, dynamic> json) =>
      _$BlockExplorerFromJson(json);
}

const kTurkiumExplorerMainnet = BlockExplorer(
  network: TurkiumNetwork.mainnet,
  name: 'Turkium Explorer',
  url: 'explorer.turkium.io',
  addressUrl: 'https://explorer.turkium.io/addresses/$kParamPattern',
  txUrl: 'https://explorer.turkium.io/txs/$kParamPattern',
);

const kTurkiumExplorerTestnet10 = BlockExplorer(
  network: TurkiumNetwork.testnet,
  name: 'Turkium Explorer',
  url: 'explorer-tn10.turkium.io',
  addressUrl: 'https://explorer-tn10.turkium.io/addresses/$kParamPattern',
  txUrl: 'https://explorer-tn10.turkium.io/txs/$kParamPattern',
);

const kTurkiumExplorerTestnet11 = BlockExplorer(
  network: TurkiumNetwork.testnet,
  name: 'Turkium Explorer',
  url: 'explorer-tn11.turkium.io',
  addressUrl: 'https://explorer-tn11.turkium.io/addresses/$kParamPattern',
  txUrl: 'https://explorer-tn11.turkium.io/txs/$kParamPattern',
);

const kKasFyiMainnet = BlockExplorer(
  network: TurkiumNetwork.mainnet,
  name: 'Kas.fyi',
  url: 'kas.fyi',
  addressUrl: 'https://kas.fyi/address/$kParamPattern',
  txUrl: 'https://kas.fyi/transaction/$kParamPattern',
);

const kKasFyiTestnet = BlockExplorer(
  network: TurkiumNetwork.testnet,
  name: 'Kas.fyi Testnet',
  url: 'testnet.kas.fyi',
  addressUrl: 'https://testnet.kas.fyi/address/$kParamPattern',
  txUrl: 'https://testnet.kas.fyi/transaction/$kParamPattern',
);

const kKatnipMainnet = BlockExplorer(
  network: TurkiumNetwork.mainnet,
  name: 'Katnip',
  url: 'katnip.kaspad.net',
  addressUrl: 'https://katnip.kaspad.net/addr/$kParamPattern',
  txUrl: 'https://katnip.kaspad.net/tx/$kParamPattern',
);

const kKatnipTestnet = BlockExplorer(
  network: TurkiumNetwork.testnet,
  name: 'Katnip Testnet',
  url: 'katnip-testnet.kaspad.net',
  addressUrl: 'https://katnip-testnet.kaspad.net/addr/$kParamPattern',
  txUrl: 'https://katnip-testnet.kaspad.net/tx/$kParamPattern',
);

const kBlockExplorersOptions = <String, List<BlockExplorer>>{
  kTurkiumNetworkIdMainnet: [
    kTurkiumExplorerMainnet,
    kKasFyiMainnet,
  ],
  kTurkiumNetworkIdTestnet10: [
    kTurkiumExplorerTestnet10,
  ],
  kTurkiumNetworkIdTestnet11: [
    kTurkiumExplorerTestnet11,
  ],
  kTurkiumNetworkIdDevnet: [],
  kTurkiumNetworkIdSimnet: [],
};
