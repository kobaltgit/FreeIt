import 'package:flutter/foundation.dart';

enum AppLang { ru, en }

final ValueNotifier<AppLang> currentLang = ValueNotifier<AppLang>(AppLang.ru);

void toggleLanguage() {
  currentLang.value = currentLang.value == AppLang.ru ? AppLang.en : AppLang.ru;
}

class Strings {
  static bool get isRussian => currentLang.value == AppLang.ru;

  static String get(String ruText, String enText) {
    return currentLang.value == AppLang.ru ? ruText : enText;
  }
}
