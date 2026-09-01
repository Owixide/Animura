# **RU — RUSSIAN**

# Animura

**Animura** — бесплатная программа для постановки анимированных обоев на **Windows 10 x64**, а в дальнейшем - **Windows 11 x64**.

> Поддержка Windows 11 пока отсутствует.

## Системные требования

Для комфортной работы:

- **ОС:** Windows 10 x64
- **Оперативная память:** 4 ГБ
- **Разрешение экрана:** 1920×1080

## Как это работает

Нужный порядок действий для запуска:

1. Вы скачиваете `Animura.exe`.
2. Запускаете его один раз.
3. Программа создаёт конфигурационные файлы и завершается. Это стандартное поведение, переживать не стоит.
4. Либо кладёте видео рядом с программой, либо указываете путь к видео в `config.txt`.
5. Запускаете программу снова.

## Первый запуск

1. Скачайте `Animura.exe` в GitHub или Telegram под необходимый релиз.
2. Желательно сразу положить его в постоянную папку, так как при запуске будут созданы нужные файлы для работы. Тем не менее, при перемещении они просто будут созданы с нуля и это не сломает приложение.
3. Запустите `Animura.exe`.
4. Программа завершится.

После первого запуска рядом с `Animura.exe` должна появиться папка `bin`, внутри которой содержится папка `settings`.

## Наиболее простой вариант запуска приложения

Если не хотите трогать конфигурационные файлы, можно сделать следующее:

1. Положите ваше медиа в то же место, где находится `Animura.exe`.
2. Переименуйте его в `video.mp4` (формат `.mp4` обязателен, так как он установлен по умолчанию).
3. Запустите `Animura.exe`.

После этого приложение будет работать полностью штатно, но это ограничивает вас в доступных форматах. Рекомендую править config, это предельно легко.

## Если видео лежит в другой папке или имеет другой формат

Например, медиа находится по пути:

`C:\MyLiveClips\Clip.mp4`

Путь к данному медиа легко прописывается в config, приложение будет работать штатно. В том числе, вы будете иметь возможность не ограничивать себя единым `.mp4`.

Править config необходимо следующим образом: 

1. Скопируйте полный путь к вашему видеофайлу.
2. Откройте папку, где лежит `Animura.exe`.
3. Перейдите в папку `bin`, затем в папку `settings`. 
4. Найдите файл `config.txt` и откройте в любом текстовом редакторе, например Блокнот или WordPad. 
5. Найдите поле `filename`, по умолчанию значение установлено на `video.mp4`.
6. Замените значение на своё, после чего не забудьте сохранить, не меняя названия, иначе всё было напрасно.

Например, если ваше медиа находится по пути `C:\MyLiveClips\Clip.mp4`, то итоговым значением в config будет являться `filename: C:\MyLiveClips\Clip.mp4`. 
Далее можете смело запускать `Animura.exe` и наслаждаться тем, что ваши обои теперь - не статичное изображение. 

## Что означают поля в `config.txt?`

> ### `width`

`width` — ширина экрана.

В этот параметр устанавливается ширина вашего экрана в пикселях.

Если указать неправильное значение, изображение может выводиться не в таком качесте или разрешении, которое вы ожидали. Крайне не рекомендуем трогать этот параметр. Он вам будет полезен только в том случае, если вы будете менять разрешение монитора.

> ### `heigth`

`heigth` — высота экрана.

В этот параметр устанавливается ширина вашего экрана в пикселях.

Если указать неправильное значение, изображение может выводиться не в таком качесте или разрешении, которое вы ожидали. Крайне не рекомендуем трогать этот параметр. Он вам будет полезен только в том случае, если вы будете менять разрешение монитора.

> ### `latency`

`latency` — межкадровая задержка.

Этот параметр влияет на скорость смены кадров.

- Если увеличить значение, видео будет идти медленнее, частота кадров будет ниже, а нагрузка уменьшится.
- Если уменьшить значение, видео будет идти быстрее, частота кадров будет выше, а нагрузка увеличится.
- Если вы установите значение параметра на `0`, то программа будет оказывать максимальную нагрузку на ваш GPU, так как нет ограничений частоты смены кадра.

Если видео кажется слишком быстрым или слишком медленным, настраивайте именно `latency`.

Важно: если используется `latency` больше нуля, параметр `vsync` должен быть равен `0`.

> ### `filename`

`filename` — путь к медиа.

Именно этот параметр говорит программе, какой файл использовать.

Если по данному пути медиа не найдено, либо повреждено или имеет недопустимый\неподдерживаемый формат, то программа упадёт с ошибкой. Настоятельно рекомендую, для личного спокойствия: кладите ваше медиа в ту же директорию, что и исполняемый `Animura.exe`, это позволит избежать множество ошибок.

> ### `vsync`

`vsync` — параметры работы VSYNC, как ни странно.

Параметр ограничивает частоту кадров относительно монитора. Возможны значения от `0` до `4`.

- `0` — без ограничения через vsync.
- `1` — частота кадров будет примерно равна частоте монитора.
- `2` — примерно половина частоты монитора.
- `3` — примерно треть частоты монитора.
- `4` — примерно четверть частоты монитора.

Чем выше значение, тем меньше кадров в секунду и тем ниже нагрузка на аппарутуру.

Важно: если используется `vsync` больше нуля, параметр `latency` должен быть равен `0`.

## Нагрузка и комфорт

Программа рассчитана на то, чтобы работать с минимально возможной нагрузкой на видеокарту, но итоговая нагрузка зависит от вашего видео, разрешения экрана и производительности системы, а так же настроек в конфигурации.

Если Animura сильно нагружает компьютер:

- увеличьте `latency`;
- или поставьте `vsync=2`, `vsync=3` либо `vsync=4`;
- используйте видео меньшего разрешения;

Важно: напоминаю, что: если используется `vsync` больше нуля, параметр `latency` должен быть равен `0`, а так же: если используется `latency` больше нуля, параметр `vsync` должен быть равен `0`.

## Временное завершение работы программы

Пока программа не имеет встроенной возможности автоматически себя завершить. Чтобы завершить исполнение, достаточно снять задачу из диспетчера задач. 

## Обратная связь

### Этот проект создан и поддерживается только лишь одним человеком. Мне было бы крайне приятно знать, что кто-либо пользуется моим приложением, мне важно узнать ваше мнение. 

### Вы можете поддержать меня, подписавшись на мой Telegram канал: https://t.me/KernelaStudio.

### Прямое общение: https://t.me/Mr_Zefy.

## Прошу, не забудьте прочесть лицензионный документ.

---

# **EN — ENGLISH**

# Animura

**Animura** is a free program for setting animated wallpapers on **Windows 10 x64**, and in the future — **Windows 11 x64**.

> Support for Windows 11 is currently unavailable.

## System Requirements

For smooth operation:

- **OS:** Windows 10 x64
- **RAM:** 4 GB
- **Screen Resolution:** 1920×1080

## How It Works

The required sequence of actions for launching:

1. You download `Animura.exe`.
2. You run it once.
3. The program creates configuration files and exits. This is standard behavior, there is no need to worry.
4. Either place the video in the same directory as the program, or specify the path to the video in `config.txt`.
5. Run the program again.

## First Launch

1. Download `Animura.exe` from GitHub or Telegram for the required release.
2. It is advisable to place it in a permanent folder right away, as the necessary files for operation will be created upon launch. Nevertheless, if moved, they will simply be created from scratch and this will not break the application.
3. Run `Animura.exe`.
4. The program will exit.

After the first launch, a `bin` folder should appear next to `Animura.exe`, which contains a `settings` folder inside.

## The Simplest Way to Launch the Application

If you do not want to touch the configuration files, you can do the following:

1. Place your media in the same location as `Animura.exe`.
2. Rename it to `video.mp4` (the `.mp4` format is mandatory, as it is set by default).
3. Run `Animura.exe`.

After this, the application will work completely normally, but this limits you in available formats. I recommend editing the config, it is extremely easy.

## If the Video is in Another Folder or Has a Different Format

For example, the media is located at the path:

`C:\MyLiveClips\Clip.mp4`

The path to this media is easily specified in the config, and the application will work normally. Among other things, you will have the ability to not limit yourself to a single `.mp4` format.

The config must be edited as follows: 

1. Copy the full path to your video file.
2. Open the folder where `Animura.exe` is located.
3. Go to the `bin` folder, then to the `settings` folder. 
4. Find the `config.txt` file and open it in any text editor, such as Notepad or WordPad. 
5. Find the `filename` field, by default the value is set to `video.mp4`.
6. Replace the value with your own, and then do not forget to save it without changing the name, otherwise it was all in vain.

For example, if your media is located at the path `C:\MyLiveClips\Clip.mp4`, then the final value in the config will be `filename: C:\MyLiveClips\Clip.mp4`. 
Then you can safely run `Animura.exe` and enjoy the fact that your wallpaper is now not a static image. 

## What do the fields in `config.txt` mean?

> ### `width`

`width` — screen width.

This parameter sets the width of your screen in pixels.

If an incorrect value is specified, the image may be displayed in a quality or resolution other than what you expected. We strongly advise against touching this parameter. It will only be useful to you if you change the monitor resolution.

> ### `heigth`

`heigth` — screen height.

This parameter sets the width of your screen in pixels.

If an incorrect value is specified, the image may be displayed in a quality or resolution other than what you expected. We strongly advise against touching this parameter. It will only be useful to you if you change the monitor resolution.

> ### `latency`

`latency` — interframe delay.

This parameter affects the frame change rate.

- If you increase the value, the video will play slower, the frame rate will be lower, and the load will decrease.
- If you decrease the value, the video will play faster, the frame rate will be higher, and the load will increase.
- If you set the parameter value to `0`, the program will put maximum load on your GPU, as there are no limits on the frame change rate.

If the video seems too fast or too slow, adjust `latency` specifically.

Important: if a `latency` greater than zero is used, the `vsync` parameter must be set to `0`.

> ### `filename`

`filename` — path to the media.

This is the parameter that tells the program which file to use.

If the media is not found at this path, is corrupted, or has an invalid\unsupported format, the program will crash with an error. I strongly recommend, for your own peace of mind: place your media in the same directory as the `Animura.exe` executable, this will allow you to avoid many errors.

> ### `vsync`

`vsync` — VSYNC operating parameters, strangely enough.

The parameter limits the frame rate relative to the monitor. Possible values are from `0` to `4`.

- `0` — no limit via vsync.
- `1` — the frame rate will be approximately equal to the monitor's refresh rate.
- `2` — approximately half the monitor's refresh rate.
- `3` — approximately one third of the monitor's refresh rate.
- `4` — approximately one quarter of the monitor's refresh rate.

The higher the value, the fewer frames per second and the lower the load on the hardware.

Important: if a `vsync` greater than zero is used, the `latency` parameter must be set to `0`.

## Load and Comfort

The program is designed to run with the lowest possible load on the graphics card, but the final load depends on your video, screen resolution, and system performance, as well as the configuration settings.

If Animura heavily loads the computer:

- increase `latency`;
- or set `vsync=2`, `vsync=3`, or `vsync=4`;
- use a lower resolution video;

Important: I remind you that: if a `vsync` greater than zero is used, the `latency` parameter must be set to `0`, and also: if a `latency` greater than zero is used, the `vsync` parameter must be set to `0`.

## Temporary Termination of the Program

Currently, the program does not have a built-in ability to automatically terminate itself. To stop execution, it is enough to end the task from the Task Manager. 

## Feedback

### This project is created and maintained by only one person. It would be extremely pleasing for me to know that someone is using my application, it is important for me to know your opinion. 

### You can support me by subscribing to my Telegram channel: https://t.me/KernelaStudio.

### Direct communication: https://t.me/Mr_Zefy.

## Please, do not forget to read the license document.

---
