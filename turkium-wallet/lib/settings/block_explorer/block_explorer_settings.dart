import 'package:freezed_annotation/freezed_annotation.dart';

import '../../turkium/turkium.dart';
import 'block_explorers.dart';

part 'block_explorer_settings.freezed.dart';
part 'block_explorer_settings.g.dart';

@freezed
sealed class BlockExplorerSettings with _$BlockExplorerSettings {
  const BlockExplorerSettings._();

  const factory BlockExplorerSettings({
    @Default(const {
      kTurkiumNetworkIdMainnet: kTurkiumExplorerMainnet,
      kTurkiumNetworkIdTestnet10: kTurkiumExplorerTestnet10,
      kTurkiumNetworkIdTestnet11: kTurkiumExplorerTestnet11,
    })
    Map<String, BlockExplorer> selection,
  }) = _BlockExplorerSettings;

  BlockExplorer explorerForNetwork(String networkId) {
    return selection[networkId] ?? kTurkiumExplorerMainnet;
  }

  factory BlockExplorerSettings.fromJson(Map<String, dynamic> json) =>
      _$BlockExplorerSettingsFromJson(json);
}
