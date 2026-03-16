# Cyrillic Comments and Strings - Safe i18n
# Legitimate Russian/Cyrillic text in code - should NOT be flagged

# Russian comments explaining functionality
# Эта функция проверяет пользователя
# (This function checks the user)

def проверить_пользователя(пользователь):
    """
    Проверяет данные пользователя
    (Validates user data)
    """
    return пользователь is not None

# Russian variable names (common in Russian projects)
имя = "Иван"  # "imya" = name
возраст = 25  # "vozrast" = age
город = "Москва"  # "gorod" = city

# Russian function names
def получить_данные():  # "poluchit_dannye" = get data
    return {"имя": имя, "возраст": возраст}

# Russian string literals
приветствие = "Привет, мир!"  # "Hello, world!"
сообщение = "Доброе утро!"  # "Good morning!"

# Russian in configuration
настройки = {
    "язык": "русский",  # "language: Russian"
    "регион": "Россия",  # "region: Russia"
    "валюта": "RUB"  # "currency"
}

# Ukrainian text (also Cyrillic)
український_текст = "Привіт, Україно!"  # "Hello, Ukraine!"

# Bulgarian text
български_текст = "Здравей, свят!"  # "Hello, world!"
