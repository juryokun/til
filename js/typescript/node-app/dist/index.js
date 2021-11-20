"use strict";
const printLine = (text, breakLine = true) => {
    process.stdout.write(text + (breakLine ? '\n' : ''));
};
const promptInput = async (text) => {
    printLine(`\n${text}\n> `, false);
    const input = await new Promise((resolve) => process.stdin.once('data', (data) => resolve(data.toString())));
    return input.trim();
};
class HintAndBlow {
    constructor() {
        this.answerSource = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        this.answer = [];
        this.tryCount = 0;
    }
}
;
(async () => {
    const hintAndBlow = new HintAndBlow();
})();
