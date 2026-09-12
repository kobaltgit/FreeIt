// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Если уже запущен экземпляр FreeIt, передаем аргументы (например, путь из контекстного меню Проводника) и выходим
    if freeit_lib::single_instance::handle_potential_secondary_instance() {
        return;
    }

    freeit_lib::run();
}