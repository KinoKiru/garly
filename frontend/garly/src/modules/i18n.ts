import type { UserModule } from './../types/uncommon';
import type { I18n } from 'vue-i18n';
import { createI18n } from 'vue-i18n';

/*
 * All i18n resources specified in the plugin `include` option can be loaded
 * at once using the import syntax
 */
import messages from '@intlify/unplugin-vue-i18n/messages';

export let i18n: I18n<
Record<string, any>,
Record<string, Record<string, Intl.DateTimeFormatOptions>>,
unknown,
false
>;

export const install: UserModule = ({ app }) => {
    const locale = localStorage.getItem('language') ?? 'nl';

    const datetimeFormat: Record<string, Intl.DateTimeFormatOptions> = {
        short: {
            month: 'long',
            day: 'numeric',
            year: 'numeric',
            // Weekday: 'long',
        },
        numeric: {
            month: '2-digit',
            day: '2-digit',
            year: 'numeric',
        },
        shortDay: {
            month: 'long',
            day: 'numeric',
            weekday: 'long',
        },
        long: {
            month: 'long',
            day: 'numeric',
            weekday: 'long',
            year: 'numeric',
        },
        shortTime: {
            month: 'long',
            day: 'numeric',
            weekday: 'long',
            hour: '2-digit',
            minute: '2-digit',
        },
    };
    i18n = createI18n({
        legacy: false,
        locale,
        messages,
        datetimeFormats: Object.fromEntries(
            Object.keys(messages).map((key) => [key, datetimeFormat]),
        ),
    });

    app.use(i18n);
};
