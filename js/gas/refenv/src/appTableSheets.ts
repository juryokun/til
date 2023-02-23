class AppTableSheets {
    sheets: Array<AppTableSheet> = Array();

    constructor(appSettings: AppSettings) {
        for (const setting of appSettings.getSettings()) {
            const sheet = new AppTableSheet();
            sheet.setData(setting);

            this.sheets.push(sheet);
        }
    }
}

interface Data { [key: number]: Array<string> }
interface DataType {type: string, value: string}

class AppTableSheet {

    sheetName: string = 'default';
    sheet: GoogleAppsScript.Spreadsheet.Sheet;
    data: Data = {};
    keyColumn: string;
    startColumn: string = 'B';
    endColumn: string = 'I';
    startRow: number = 3;
    endRow: number;

    constructor(sheetName: string | null = null) {
        const spreadSheet = SpreadsheetApp.getActiveSpreadsheet();
        const sheet = spreadSheet.getSheetByName(sheetName !== null ? sheetName : this.sheetName);
        if (sheet === null) {
            return;
        }

        this.sheet = sheet;
        this.endRow = this.sheet.getLastRow()+1;
        this.setKeyColumn();
    }

    setData(appSetting: AppSetting) {
        const maps = this.getColumnMaps();

        const tableRow = Array();
        let key = '';
        for (const map of maps) {
            if (map.type === 'key') {
                key = this.getSettingValue(appSetting, map);
            }
            tableRow.push(this.getSettingValue(appSetting, map));
        }
        const row = this.getInsertRow(key);
        this.data[row] = tableRow;
    }
    getSettingValue(appSetting: AppSetting, data: DataType) {
        if (data.type === 'schema') {
            const connections = appSetting.getDbconnections();
            return connections.getConnectionShcema(data.value);
        }

        switch (data.value) {
            case 'app':
                return appSetting.getAppInfo().getApp();
            case 'env':
                return appSetting.getEnv();
            case 'repository':
                return appSetting.getRepository();
            case 'message':
                return appSetting.getMessage();
            case 'limit':
                return appSetting.getLimit();
            case 'type':
                return appSetting.getAppInfo().getType();
            case 'subApp':
                return appSetting.getAppInfo().getSubApp();
            default:
                return '';
        }
    }

    getColumnMaps() {
        const maps: Array<DataType> = Array(
            {type: 'key', value: 'env'},
            {type: '', value: 'repository'},
            {type: '', value: 'message'},
            {type: '', value: 'limit'},
            {type: '', value: 'type'},
            {type: '', value: 'subApp'},
            {type: 'schema', value: 'devSchema'},
            {type: 'schema', value: 'testSchema'},
        );
        return maps;
    }

    convertFromAlphabetToNumber(letter: string) {
        let num: number = 0;
        let temp: number = 0;

        letter = letter.toUpperCase();
        for (let i = letter.length - 1; i >= 0; i--) {
            temp = letter.charCodeAt(i) - 65;
            if(i != letter.length - 1) {
                temp = (temp + 1) * Math.pow(26,(i + 1));
            }
            num = num + temp
        }
        return num + 1;
    }
    convertFromNumberToAlphabet = (num: number): string => {
        let temp: number, letter: string = '';
        while (num > 0) {
            temp = (num - 1) % 26;
            letter = String.fromCharCode(temp + 65) + letter;
            num = (num - temp - 1) / 26;
        }
        return letter;
    }
    setKeyColumn() {
        let i = 0;
        for (const column of this.getColumnMaps()) {
            if (column.type === 'key') {
                this.keyColumn = this.convertFromNumberToAlphabet(i + this.convertFromAlphabetToNumber(this.startColumn));
                return;
            }
            i++;
        }
        this.keyColumn = this.startColumn;
    }
    getInsertRow(key: string) {
        for (let row = this.startRow; row <= this.endRow; row++) {
            const range = this.keyColumn + row;
            if (this.sheet.getRange(range).getValue() == key) {
                return row;
            }
        }
        const returnRow = this.endRow;
        this.endRow += 1;

        return returnRow;
    }

    writeSpreadSheet() {
        for (const key of Object.keys(this.data)) {
            const dataColumns = this.startColumn + key + ':' + this.endColumn + key;
            const range = this.sheet.getRange(dataColumns);

            range.setValues([this.data[key]]);
        }
    }
}

class PasnaviTableSheet extends AppTableSheet {

    startColumn: string = 'B';
    endColumn: string = 'H';
    constructor() {
        super('pasnavi');
    }

    getColumnMaps() {
        const maps: Array<DataType> = Array(
            {type: 'key', value: 'env'},
            {type: '', value: 'repository'},
            {type: '', value: 'message'},
            {type: '', value: 'limit'},
            {type: '', value: 'type'},
            // {type: '', value: 'subApp'},
            {type: 'schema', value: 'devSchema'},
            {type: 'schema', value: 'testSchema'},
        );
        return maps;
    }
}

const testDataSet = () => {
    const dataClass = new TestData1();
    const data = dataClass.getTestData();

    const appSettings = new AppSettings();
    const loadedData = appSettings.loadFromJsonStr(data);
    const sheet = new AppTableSheet();

    for (const setting of loadedData) {
        sheet.setData(setting);
    }
    sheet.writeSpreadSheet();

    const pasSheet = new PasnaviTableSheet();
    for (const pasSetting of loadedData) {
        pasSheet.setData(pasSetting);
    }
    pasSheet.writeSpreadSheet();

    // let message = "";
    // message += customAssertion(sheet.data[0]['AA'][0], "pasnavi");
    // if (message.length > 0) {
    //     Logger.log(message);
    // } else {
    //     Logger.log("Success!!");
    // }
}

