class AppTableSheets {
    sheets: Array<AppTableSheet>;

    constructor() {
        this.sheets = new Array();
    }
}

interface Data { [key: string]: Array<string> };

class AppTableSheet {

    sheetName: string = 'default';
    sheet: GoogleAppsScript.Spreadsheet.Sheet | null;
    data: Array<Data>;

    constructor() {
        const spreadSheet = SpreadsheetApp.getActiveSpreadsheet();
        this.sheet = spreadSheet.getSheetByName(this.sheetName);
    }

    setData(appSetting: AppSetting) {
        const maps = this.getMaps();

        this.data = Array();
        for (const map of maps) {
            this.data.push(appSetting[map])
        }
    }

    getMaps() {
        const maps = Array();
        return maps;
    }
    writeSpreadSheet() {
    }
}
