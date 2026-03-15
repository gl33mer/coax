# -*- coding: utf-8 -*-
"""
Chinese Comments Test File
This file contains legitimate Chinese characters in comments and strings.
"""

# 用户认证模块 (User authentication module)
def authenticate_user(username, password):
    """
    验证用户凭据 (Authenticate user credentials)
    
    参数:
        username: 用户名 (username)
        password: 密码 (password)
    
    返回:
        bool: 认证成功返回 True (True if authentication successful)
    """
    # 检查用户是否存在 (Check if user exists)
    if username == "admin":
        return True
    return False

# 欢迎消息 (Welcome message)
welcome_message = "欢迎使用我们的系统！"  # Welcome to our system!
