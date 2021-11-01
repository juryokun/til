const envColmunPositon = 2; // B
const pasnaviColumnPosition = 5; // E
const enaviColumnPosition = 9; // I
const pcRowPosition = 6;
const iPadRowPosition = 24;


function doPost(params) {
  try {
    let data = JSON.parse(params.postData.getDataAsString());
    Logger.log(data);
    setInformation_(data);

  } catch(error) {
    return ContentService.createTextOutput("エラーが発生しました。"+error);
  }
  return ContentService.createTextOutput("Success!!");
}

function setInformation_(data) {
  // setting sheet
  let sheet = SpreadsheetApp.getActive().getSheetByName('summary');

  // set variable
  let env = data.env;
  let app = data.app;

  let targetRow = getTargetRow_(sheet, env, app);

  if (app == "pasnavi") {
    writePasnavi_(sheet, data.pasnavi, targetRow);
  }

  if (app == "enavi") {
    writeEnavi_(sheet, data.enavi, targetRow);
  }
  
  if (app == "enavi2") {
    writeEnavi2_(sheet, data.enavi2, targetRow);
  }
}


function getTargetRow_(sheet, targetEnv, app) {
  let headRow = pcRowPosition;
  if (app == "pasapi") {
    headRow = iPadRowPosition;
  }
  
  let targetRow = null;

  let lastRow = sheet.getRange(headRow, envColmunPositon).getNextDataCell(SpreadsheetApp.Direction.DOWN).getRow();
  for(let i = headRow; i <= lastRow; i++) {
    var env = sheet.getRange(i, envColmunPositon).getValue();
    if (env == targetEnv) {
      targetRow = i;
      break;
    }
  }

  return targetRow;
}

function writePasnavi_(sheet, setting, row) {
  writeCell_(sheet,row,pasnaviColumnPosition, setting.svn);
  writeCell_(sheet,row,pasnaviColumnPosition+1, setting.pasnaviDb);
  writeCell_(sheet,row,pasnaviColumnPosition+2, setting.enaviDb);
  writeCell_(sheet,row,pasnaviColumnPosition+3, today_())
}

function writeEnavi_(sheet, setting, row) {
  writeCell_(sheet,row,enaviColumnPosition+1, setting.svn);
  writeCell_(sheet,row,enaviColumnPosition+2, setting.enaviDb);
  writeCell_(sheet,row,enaviColumnPosition+3, today_())
}

function writeEnavi2_(sheet, setting, row) {
  writeCell_(sheet,row,enaviColumnPosition, setting.svn);
  writeCell_(sheet,row,enaviColumnPosition+3, today_())
}

function writeCell_(sheet, row, column, value) {
  sheet.getRange(row, column).setValue(value);
}

function today_() {
  let date = new Date();
  return Utilities.formatDate( date, 'Asia/Tokyo', 'yyyy/MM/dd');
}

function myFunc() {
  Logger.log(DriveApp.getRootFolder().getName());
  Logger.log(ScriptApp.getOAuthToken());
}
