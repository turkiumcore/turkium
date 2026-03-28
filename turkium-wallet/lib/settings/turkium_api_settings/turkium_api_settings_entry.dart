import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/core_providers.dart';
import '../../settings_drawer/double_line_item.dart';
import '../../widgets/sheet_util.dart';
import '../setting_item.dart';
import 'turkium_api_settings_providers.dart';
import 'turkium_api_settings_sheet.dart';

class TurkiumApiSettingsUrlEntry extends ConsumerWidget {
  const TurkiumApiSettingsUrlEntry({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final turkiumApiUrl = ref.watch(turkiumApiUrlProvider);

    void _changeTurkiumApiUrl() {
      final theme = ref.read(themeProvider);
      Sheets.showAppHeightEightSheet(
        context: context,
        theme: theme,
        widget: const TurkiumApiSettingsSheet(),
      );
    }

    return DoubleLineItem(
      heading: 'Turkium API',
      defaultMethod: StringSelectionItem(
        turkiumApiUrl.substring(turkiumApiUrl.indexOf('://') + 3),
      ),
      icon: Icons.api,
      onPressed: _changeTurkiumApiUrl,
    );
  }
}
