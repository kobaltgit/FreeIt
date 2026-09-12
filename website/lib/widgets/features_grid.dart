import 'package:flutter/material.dart';
import '../theme.dart';
import '../i18n.dart';

class FeaturesGrid extends StatelessWidget {
  const FeaturesGrid({super.key});

  @override
  Widget build(BuildContext context) {
    final isDark = siteThemeMode.value == ThemeMode.dark;

    final features = [
      {
        'icon': Icons.search,
        'titleRu': 'Restart Manager API',
        'titleEn': 'Restart Manager API',
        'descRu': 'Мгновенный вызов Win32 API для определения PID, заголовков окон и путей процессов.',
        'descEn': 'Native Win32 API calls to resolve locking PIDs, window titles, and file handles.',
      },
      {
        'icon': Icons.shield_outlined,
        'titleRu': 'Индикатор риска (🟢/🟡/🔴)',
        'titleEn': 'Smart Risk Indicators',
        'descRu': 'Цветовая маркировка степеней риска — от обычных программ до защищенных системных служб.',
        'descEn': 'Color-coded process risk levels: Safe user apps, Background services, or System kernel.',
      },
      {
        'icon': Icons.auto_awesome,
        'titleRu': 'ИИ-Инспектор Gemini',
        'titleEn': 'Gemini AI Insights',
        'descRu': 'Экспертный ИИ-разбор причин блокировки, назначения файла и рекомендации по безопасности.',
        'descEn': 'AI analysis explaining why the file is locked, its purpose, and safety recommendations.',
      },
      {
        'icon': Icons.delete_outline,
        'titleRu': 'Удаление в Корзину & MiniBin',
        'titleEn': 'Recycle Bin & MiniBin Sync',
        'descRu': 'Принудительное выселение и перемещение в Корзину со слушателем `SHChangeNotifyRegister`.',
        'descEn': 'Forced file release & move to Windows Recycle Bin with instant MiniBin tray updates.',
      },
      {
        'icon': Icons.edit_note,
        'titleRu': 'Переименование на месте',
        'titleEn': 'In-Place Rename',
        'descRu': 'Мгновенная разблокировка и обновление имени файла/папки прямо из приложения.',
        'descEn': 'Instant release of handles and file/folder renaming without leaving the app.',
      },
      {
        'icon': Icons.drag_indicator,
        'titleRu': 'Drag & Drop и ПКМ',
        'titleEn': 'Drag & Drop & Shell Menu',
        'descRu': 'Перетаскивание элементов в окно утилиты и интеграция с контекстным меню Проводника.',
        'descEn': 'Native Windows Drag & Drop payload handling and Explorer context menu integration.',
      },
    ];

    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 40),
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 1100),
        child: Column(
          children: [
            Text(
              Strings.get('Ключевые возможности', 'Key Features'),
              style: TextStyle(fontSize: 28, fontWeight: FontWeight.bold, color: AppColors.textPrimary),
            ),
            const SizedBox(height: 32),
            LayoutBuilder(
              builder: (context, constraints) {
                int crossAxisCount = 3;
                if (constraints.maxWidth < 640) {
                  crossAxisCount = 1;
                } else if (constraints.maxWidth < 960) {
                  crossAxisCount = 2;
                }

                return GridView.builder(
                  shrinkWrap: true,
                  physics: const NeverScrollableScrollPhysics(),
                  gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                    crossAxisCount: crossAxisCount,
                    crossAxisSpacing: 20,
                    mainAxisSpacing: 20,
                    mainAxisExtent: 175,
                  ),
                  itemCount: features.length,
                  itemBuilder: (context, index) {
                    final f = features[index];
                    return Container(
                      padding: const EdgeInsets.all(20),
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
                                  color: Color(0x0C000000),
                                  blurRadius: 12,
                                  offset: Offset(0, 4),
                                ),
                              ],
                      ),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Icon(f['icon'] as IconData, color: AppColors.accent, size: 28),
                          const SizedBox(height: 12),
                          Text(
                            Strings.get(f['titleRu'] as String, f['titleEn'] as String),
                            style: TextStyle(
                              fontSize: 16,
                              fontWeight: FontWeight.bold,
                              color: AppColors.textPrimary,
                            ),
                            maxLines: 1,
                            overflow: TextOverflow.ellipsis,
                          ),
                          const SizedBox(height: 6),
                          Expanded(
                            child: Text(
                              Strings.get(f['descRu'] as String, f['descEn'] as String),
                              style: TextStyle(
                                fontSize: 13,
                                fontWeight: isDark ? FontWeight.normal : FontWeight.w500,
                                color: AppColors.textSecondary,
                                height: 1.4,
                              ),
                              overflow: TextOverflow.fade,
                            ),
                          ),
                        ],
                      ),
                    );
                  },
                );
              },
            ),
          ],
        ),
      ),
    );
  }
}
