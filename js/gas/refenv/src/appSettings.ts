class AppSettings {
    settings: Array<AppSetting>;

    constructor() {
        this.settings = new Array();
    }
    loadFromJsonStr(json_str: string) {
        const datas = JSON.parse(json_str);
        for (const data of datas) {
            this.settings.push(new AppSetting(data));
        }
        return this.settings;
    }
    getSettings() {
        return this.settings;
    }
}

class AppSetting {
    env: string;
    appInfo: AppInfo;
    repository: string;
    message: string;
    limit: string;
    dbConnections: DbConnections;

    constructor(obj: any) {
        this.env = this.validEnv(obj.env);
        this.appInfo = new AppInfo(obj.appInfo);
        this.repository = this.validRepository(obj.repository);
        this.message = this.validMessage(obj.message);
        this.limit = this.validLimit(obj.limit);
        this.dbConnections = new DbConnections(obj.dbConnections);
    }

    validEnv(env: string) {
        if (env.length == 0) {
            const message = "環境名(env)が設定されていません。";
            throw new ValueError(this.constructor.name, message);
        }
        return env;
    }
    validRepository(repository: string) {
        const pattern = /^https?:\/\/[\w/:%#\$&\?\(\)~\.=\+\-]+$/;
        if (!pattern.test(repository)) {
            const message = "svnリポジトリURL(repository)が設定されていません（設定値：" + repository + "）。";
            throw new ValueError(this.constructor.name, message);
        }
        return repository;
    }
    validMessage(message: string) {
        // 必須ではないのでチェックしない
        return message;
    }
    validLimit(limit: string) {
        // 必須ではないのでチェックしない
        return limit;
    }

    getEnv() {
        return this.env;
    }
    getRepository() {
        return this.repository;
    }
    getMessage() {
        return this.message;
    }
    getLimit() {
        return this.limit;
    }
    getAppInfo() {
        return this.appInfo;
    }
    getDbconnections() {
        return this.dbConnections;
    }
}

class AppInfo {
    app: string;
    type: string;
    subApp: string;

    constructor(obj: any) {
        this.app = this.validApp(obj.app);
        this.type = this.validType(obj.type);
        this.subApp = this.validSubApp(obj.subApp);
    }
    validApp(app: string) {
        switch (app) {
            case "pasnavi":
            case "enavi":
                break;
        default:
            const message = "アプリ名(app)が適切ではありません。";
            throw new ValueError(this.constructor.name, message);
            break;
        }

        return app;
    }
    validType(type: string) {
        switch (type) {
            case "web":
            case "batch":
                break;
        default:
            const message = "web or batch(tye)が設定されていません（設定値：" + type + "）。";
            throw new ValueError(this.constructor.name, message);
            break;
        }

        return type;
    }
    validSubApp(subApp: string) {
        if (this.app == "enavi" && this.type == "batch" && subApp.length == 0) {
            const message = "enaviバッチの場合はバッチ名(subApp)が必要です。";
            throw new ValueError(this.constructor.name, message);
        }
        return subApp;
    }

    getApp() {
        return this.app;
    }
    getType() {
        return this.type;
    }
    getSubApp() {
        return this.subApp;
    }
}

class DbConnections {
    connections: { [schema: string]: DbConnection } = {};

    constructor(dbConnections: any) {
        for (const connection of dbConnections) {
            const dbConnection = new DbConnection(connection);
            this.connections[dbConnection.getSchema()] = dbConnection;
        }
    }
    getConnectionShcema(schema: string) {
        const connection = this.connections[schema];
        return connection?.getConnectionSchema();
    }
}

class DbConnection {
    schema: string;
    connectionSchema: string;

    constructor(obj: any) {
        this.schema = obj.schema;
        this.connectionSchema = obj.connectionSchema;
    }
    validSchema(schema: string) {
        if (schema.length == 0) {
            const message = "スキーマ名(schema)が設定されていません。";
            throw new ValueError(this.constructor.name, message);
        }
        return schema;
    }
    validConnectionSchema(connectionSchema: string) {
        if (connectionSchema.length == 0) {
            const message = "DBの接続先(connectionSchema)が設定されていません。";
            throw new ValueError(this.constructor.name, message);
        }
        return connectionSchema;
    }

    getSchema() {
        return this.schema;
    }
    getConnectionSchema() {
        return this.connectionSchema;
    }
}

class ValueError extends Error {
    public constructor(className:string, message?: string) {
        let alertMessage = message + "(Error-class: " + className + ")";
        super(alertMessage);
  }
}

const testLoadFromJsonStr = () => {
    const data = '[{"env":"dev","appInfo":{"app":"pasnavi","type":"web","subApp":""},"repository":"https://----","message":"利用目的","limit":"2023/03/31","dbConnections":[{"schema":"testSchema","connectionSchema":"testConnectionSchema"},{"schema":"devSchema","connectionSchema":"devConnectionSchema"}]},{"env":"dev","appInfo":{"app":"enavi","type":"batch","subApp":"subApp1"},"repository":"https://----","message":"利用目的","limit":"2023/03/31","dbConnections":[{"schema":"devSchema","connectionSchema":"devSchema00"}]}]';

    const appSettings = new AppSettings();
    const loadedData = appSettings.loadFromJsonStr(data);

    let message = "";
    const data1 = loadedData[0];
    message += customAssertion(data1.env, "dev");
    message += customAssertion(data1.appInfo.app, "pasnavi");
    message += customAssertion(data1.appInfo.subApp, "");
    message += customAssertion(data1.dbConnections.getConnectionShcema("testSchema"), "testConnectionSchema");
    message += customAssertion(data1.dbConnections.getConnectionShcema("devSchema"), "devConnectionSchema");

    const data2 = loadedData[1];
    message += customAssertion(data2.env, "dev");
    message += customAssertion(data2.appInfo.app, "enavi");
    message += customAssertion(data2.appInfo.subApp, "subApp1");
    message += customAssertion(data1.dbConnections.getConnectionShcema("devSchema"), "devSchema00");

    if (message.length > 0) {
        Logger.log(message);
    } else {
        Logger.log("Success!!");
    }
}

const customAssertion = (data: string, expected: string): string =>  {
    let message = ""
    if (data != expected) {
        message = "data: " + data + "| " + "期待結果: " + expected + "\n";
    }
    return message;
}
