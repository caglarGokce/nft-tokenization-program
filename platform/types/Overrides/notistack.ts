import 'notistack';

declare module 'notistack' {
  interface VariantOverrides {
    // adds `loading` variant
    loading: true;
  }
}
