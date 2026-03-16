// Chinese Strings and Comments - Safe i18n
// Legitimate Chinese text in code - should NOT be flagged

// Chinese comments
// 这个函数处理用户数据
// (This function processes user data)

// Chinese variable names (common in Chinese projects)
const 用户 = "张三";  // "yònghù" = user
const 年龄 = 28;  // "niánlíng" = age
const 城市 = "北京";  // "chéngshì" = city

// Chinese function names
function 问候 () {  // "wènhòu" = greet
    console.log("你好，世界！");  // "Hello, world!"
}

// Chinese string literals
const 欢迎消息 = "欢迎光临！";  // "Welcome!"
const 错误消息 = "发生错误";  // "An error occurred"

// Chinese in object properties
const 配置 = {
    语言："中文",  // "language: Chinese"
    地区："中国",  // "region: China"
    货币："CNY"  // "currency"
};

// Chinese class names
class 用户 {  // "User"
    constructor(姓名，年龄) {
        this.姓名 = 姓名;
        this.年龄 = 年龄;
    }
}

// Traditional Chinese
const 傳統中文 = "傳統中文文字";  // "Traditional Chinese text"

// Mixed Chinese and English (common in i18n)
const user_信息 = { name: "Li", 状态："active" };
