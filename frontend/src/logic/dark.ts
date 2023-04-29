export const isDark = useDark({
    emitAuto: false,
    initialValue: 'dark',
});
export const toggleDark = useToggle(isDark);
