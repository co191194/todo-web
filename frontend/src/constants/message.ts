/**
 * メッセージ一覧です。
 *
 * メッセージIDの形式：
 *   メッセージ種別 ＋ 4桁数字(左0埋め、全てのメッセージでの連番)
 *
 * メッセージ種別：
 *   I：情報
 *   W：警告
 *   E：エラー
 *
 */
export const Message = {
  E0001: '有効な{0}を入力してください。',
  E0002: '{0}は{1}文字以上で入力してください。',
  E0003: '{0}が一致しません。',
  E0004: '{0}または{1}が正しくありません。',
  E0005: '{0}に失敗しました。しばらく経ってからお試しください。',
  E0006: 'この{0}は既に登録されています。'
} as const satisfies Record<string, string>;

/**
 * メッセージを作成します。
 * 
 * @param message メッセージ
 * @param args 埋め込む値
 * @returns 
 */
export function getMessage(
  message: typeof Message[keyof typeof Message],
  ...args: string[]
): string {
  return message.replace(/\{(\d+)\}/g, (match, number) => {
    return typeof args[number] !== 'undefined' ? args[number] : match;
  });
}
