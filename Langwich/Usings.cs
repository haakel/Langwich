// ──────────────────────────────────────────────────────────────────
// رفع تداخل اسمی بین System.Windows (WPF) و System.Windows.Forms
// وقتی هر دو UseWPF و UseWindowsForms با ImplicitUsings فعال باشند.
// ──────────────────────────────────────────────────────────────────
global using Application  = System.Windows.Application;
global using KeyEventArgs = System.Windows.Input.KeyEventArgs;
global using MessageBox   = System.Windows.MessageBox;
global using Clipboard    = System.Windows.Clipboard;
