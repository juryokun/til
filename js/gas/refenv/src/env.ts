function myFunction() {
    var obj = [
        {
            "env": "dev",
            "appInfo": {
                "app": "pasnavi",
                "type": "web",
                "subApp": ""
            },
            "svn": "https://----",
            "message": "利用目的",
            "limit": "2023/03/31",
            "dbConnections": [
                {
                    "schema": "pasnavi",
                    "connectionSchema": "target_pasnavi",
                },
                {
                    "schema": "enavi",
                    "connectionSchema": "target_enavi",
                }
            ]
        },
        {
            "env": "dev",
            "appInfo": {
                "app": "enavi",
                "type": "batch",
                "subApp": "target_tok_batch"
            },
            "svn": "https://----",
            "message": "利用目的",
            "limit": "2023/03/31",
            "dbConnections": [
                {
                    "schema": "enavi",
                    "connectionSchema": "target_enavi",
                }
            ]
        }
    ];
    var json = JSON.stringify(obj)
    Logger.log(json)
}
