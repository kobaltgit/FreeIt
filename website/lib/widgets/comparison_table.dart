import 'package:flutter/material.dart';
import '../theme.dart';
import '../i18n.dart';

class ComparisonTable extends StatelessWidget {
  const ComparisonTable({super.key});

  @override
  Widget build(BuildContext context) {
    final isDark = siteThemeMode.value == ThemeMode.dark;
    final highlightGreen = isDark ? const Color(0xFF34D399) : const Color(0xFF047857);
    final highlightCyan = isDark ? const Color(0xFF38BDF8) : const Color(0xFF0369A1);

    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 40),
      child: Column(
        children: [
          Text(
            Strings.get('Сравнение с аналогами', 'Comparison with Competitors'),
            style: TextStyle(fontSize: 28, fontWeight: FontWeight.bold, color: AppColors.textPrimary),
          ),
          const SizedBox(height: 24),
          ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 800),
            child: Container(
              decoration: BoxDecoration(
                color: AppColors.surfaceCard,
                borderRadius: BorderRadius.circular(12),
                border: Border.all(
                  color: AppColors.borderSubtle,
                  width: isDark ? 1.0 : 1.5,
                ),
                boxShadow: isDark
                    ? null
                    : const [
                        BoxShadow(
                          color: Color(0x0F000000),
                          blurRadius: 16,
                          offset: Offset(0, 4),
                        ),
                      ],
              ),
              child: Table(
                border: TableBorder.all(
                  color: AppColors.borderSubtle,
                  width: isDark ? 0.5 : 1.5,
                ),
                children: [
                  TableRow(
                    decoration: BoxDecoration(
                      color: isDark ? const Color(0x15FFFFFF) : const Color(0xFFE2E8F0),
                    ),
                    children: [
                      _cell(Strings.get('Параметр', 'Metric'), isHeader: true),
                      _cell('FreeIt', isHeader: true, color: AppColors.accent),
                      _cell('IObit Unlocker', isHeader: true),
                      _cell('LockHunter', isHeader: true),
                    ],
                  ),
                  TableRow(
                    children: [
                      _cell(Strings.get('Стек технологий', 'Tech Stack'), isMetric: true),
                      _cell('Rust + Tauri v2 + Svelte 5', color: highlightCyan, isBold: true),
                      _cell('C++ / MFC (Legacy UI)'),
                      _cell('C++ / Win32'),
                    ],
                  ),
                  TableRow(
                    children: [
                      _cell(Strings.get('ОЗУ в фоне', 'Idle RAM'), isMetric: true),
                      _cell('< 15 МБ', color: highlightGreen, isBold: true),
                      _cell('~45–80 МБ'),
                      _cell('~40–70 МБ'),
                    ],
                  ),
                  TableRow(
                    children: [
                      _cell(Strings.get('Холодный запуск', 'Cold Launch'), isMetric: true),
                      _cell('~50 мс', color: highlightGreen, isBold: true),
                      _cell('~300–600 мс'),
                      _cell('~400–700 мс'),
                    ],
                  ),
                  TableRow(
                    children: [
                      _cell(Strings.get('ИИ-Инспектор файлов', 'AI File Analysis'), isMetric: true),
                      _cell('✅ Gemini 2.5 Flash', color: highlightGreen, isBold: true),
                      _cell('❌ Нет'),
                      _cell('❌ Нет'),
                    ],
                  ),
                  TableRow(
                    children: [
                      _cell(Strings.get('Права Admin (UAC)', 'Admin Rights (UAC)'), isMetric: true),
                      _cell('Не требуются (HKCU)', color: highlightGreen, isBold: true),
                      _cell('Обязательны UAC'),
                      _cell('Обязательны UAC'),
                    ],
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _cell(String text, {bool isHeader = false, bool isMetric = false, bool isBold = false, Color? color}) {
    final isDark = siteThemeMode.value == ThemeMode.dark;
    final defaultColor = isHeader || isMetric
        ? AppColors.textPrimary
        : AppColors.textSecondary;

    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 12),
      child: Text(
        text,
        style: TextStyle(
          fontWeight: (isHeader || isMetric || isBold) ? FontWeight.bold : (isDark ? FontWeight.normal : FontWeight.w500),
          color: color ?? defaultColor,
          fontSize: isHeader ? 13 : 12,
        ),
      ),
    );
  }
}