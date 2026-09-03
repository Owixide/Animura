# **RU | Русский**

# Animura

**Animura** — бесплатная программа для постановки анимированных обоев на **Windows 10 x64**, в дальнейшем - **Windows 11 x64**.

## Системные требования

- **ОС:** Windows 10 x64
- **Оперативная память:** 4 ГБ
- **Разрешение экрана:** 1920×1080

## Действия, порождающие априори корректный запуск:

1. Сохранить `Animura.exe` интересующей вас версии.
2. Запустите его единожды, после чего в той же директории будет создана папка `bin`. Это поведение - стандартно, не стоит переживать.

> **Важно**: папка `bin` будет пересоздана, если отсутствует, или же не имеет какого либо составляющего. Если вы переместили >исполняемый файл `Animura.exe`, то хорошим тоном будет перемещение и `bin` в то место, где в текущий момент расположен `Animura.exe`, если вы не хотите заново настраивать **config**.

3. Вам необходимо переместить конкретное медиа в папку, где лежит `Animura.exe`. 

4. Перейдите в папку `bin`. Далее, в папку `settings`. В папке `settings` будет находиться `config.txt`, который вам необходимо открыть с помощью любого текстового редактора.
5. Установите значение предпоследнего параметра в `config.txt` на название вашего медиа (или установите как абсолютный путь, в случае, если медиа по какой-либо причине находится не в одной директории с `Animura.exe`).

> **Например**: по умолчанию предпоследняя строчка имеет значение `filename: video.mp4`. Поменяйте `video.mp4` на интересующее вас медиа, либо абсолютный путь.
> **Примеры**: 
> - Пример с указанием `названия медиа`: `filename: Clip.mp4` в том случае, если ваше медиа находится **в той же** директории, что и исполняемый файл.
> - Пример с указанием `абсолютного пути до медиа`: `filename: C:\MyLiveClips\Clip.mp4` в том случае, если ваше медиа находится **не в той же** директории, что и исполняемый файл.
Если не хотите трогать конфигурационные файлы, можно сделать следующее:

6. Необходимо сохранить `config.txt` после изменений, если вы их делали.

> Обращаю ваше внимание: при удалении / потере папки `bin` (либо `config.txt`) вам придётся настраивать его заново.
> Вам необходимо править `config.txt` каждый раз, если вы хотите, например, поменять живые обои на другие | не устраивает скорость воспроизведения | нагрузка на CPU/GPU. 

## Значения полей в `config.txt`

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

Этот параметр говорит программе, какое медиа использовать.

Если по данному пути медиа не найдено, либо повреждено или имеет недопустимый | неподдерживаемый формат, то программа упадёт с ошибкой. Настоятельно рекомендую, для личного спокойствия: кладите ваше медиа в ту же директорию, что и исполняемый `Animura.exe`, это позволит избежать множество ошибок.

> ### `vsync`

`vsync` — параметры работы VSYNC, как ни странно.

Параметр ограничивает частоту кадров относительно монитора. Возможны значения от `0` до `4`.

- `0` — без ограничения частоты кадров через vsync.
- `1` — частота кадров будет стремиться к равенству частоты кадров монитора.
- `2` — частота кадров будет стремиться к 1/2 частоты кадров монитора.
- `3` — частота кадров будет стремиться к 1/3 частоты кадров монитора.
- `4` — частота кадров будет стремиться к 1/4 частоты кадров монитора.

Чем выше значение, тем меньше кадров в секунду вы получите. Следовательно, тем ниже нагрузка на машину.

Важно: если используется `vsync` больше нуля, параметр `latency` должен быть равен `0`.

## Нагрузка и комфорт

Программа рассчитана на то, чтобы работать с минимально возможной нагрузкой на видеокарту, но итоговая нагрузка зависит от вашего видео, разрешения экрана и производительности системы, а так же настроек в конфигурации.

Если **Animura** сильно нагружает машину:

- увеличьте `latency`.
- поставьте `vsync: 2`, `vsync: 3` либо `vsync: 4`.
- используйте видео меньшего разрешения.

Важно: напоминаю, что если используется `vsync` больше нуля, параметр `latency` должен быть равен `0`, а так же: если используется `latency` больше нуля, параметр `vsync` должен быть равен `0`.

## Завершение работы программы

Пока программа не имеет встроенной возможности автоматически себя завершить. Чтобы завершить исполнение, достаточно снять задачу из диспетчера задач. 

## Обратная связь

### Этот проект создан и поддерживается только лишь одним человеком. Мне было бы крайне приятно знать, что кто-либо пользуется моим приложением, мне важно узнать ваше мнение. 

### Вы можете поддержать меня, подписавшись на мой Telegram канал: https://t.me/KernelaStudio.

### Прямое общение: https://t.me/Mr_Zefy.

## Прошу, не забудьте прочесть лицензионный документ.

---

# **EN | English**

# Animura

**Animura** is a free application for setting animated wallpapers on **Windows 10 x64**, with future support for **Windows 11 x64**.

## System Requirements

- **OS:** Windows 10 x64
- **RAM:** 4 GB
- **Screen Resolution:** 1920×1080

## Steps to Ensure a Correct Launch:

1. Save the `Animura.exe` file of your required version.
2. Run it once. A `bin` folder will be created in the same directory. This is standard behavior, do not worry.

> **Important**: The `bin` folder will be recreated if it is missing or incomplete. If you move the `Animura.exe` executable, it is highly recommended to move the `bin` folder to the new location of `Animura.exe` as well, unless you want to reconfigure the **config** from scratch.

3. You need to move your specific media file into the directory containing `Animura.exe`. 

4. Navigate to the `bin` folder, then into the `settings` folder. Inside `settings`, you will find `config.txt`, which you need to open with any text editor.
5. Set the value of the `filename` parameter to the name of your media file (or provide an absolute path if the media is located outside the `Animura.exe` directory for any reason).

> **Example**: By default, the line is set to `filename: video.mp4`. Change `video.mp4` to your desired media file, or provide an absolute path.
> **Examples**: 
> - Using the `media filename`: `filename: Clip.mp4` if your media is located **in the same** directory as the executable file.
> - Using the `absolute path to media`: `filename: C:\MyLiveClips\Clip.mp4` if your media is located **in a different** directory than the executable file.
If you prefer not to edit the configuration files manually, you can do the following:

6. You must save `config.txt` after making any changes.

> Please note: if the `bin` folder (or `config.txt`) is deleted or lost, you will have to reconfigure it from scratch.
> You need to edit `config.txt` every time you want to change the live wallpaper, playback speed, or CPU/GPU load. 

## Configuration Fields in `config.txt`

> ### `width`

`width` — screen width.

Set this parameter to your screen width in pixels.

If an incorrect value is specified, the image may be rendered in a quality or resolution you did not expect. We strongly advise against modifying this parameter. It is only useful if you change your monitor's resolution.

> ### `height`

`height` — screen height.

Set this parameter to your screen height in pixels.

If an incorrect value is specified, the image may be rendered in a quality or resolution you did not expect. We strongly advise against modifying this parameter. It is only useful if you change your monitor's resolution.

> ### `latency`

`latency` — inter-frame delay.

This parameter affects the frame transition speed.

- If you increase the value, the video will play slower, the frame rate will drop, and the system load will decrease.
- If you decrease the value, the video will play faster, the frame rate will increase, and the system load will rise.
- If you set this parameter to `0`, the program will exert maximum load on your GPU, as there will be no frame rate limitations.

If the video seems too fast or too slow, adjust the `latency` parameter.

Important: If `latency` is greater than zero, the `vsync` parameter must be set to `0`.

> ### `filename`

`filename` — path to the media.

This parameter tells the program which media file to use.

If the media is not found at the specified path, is corrupted, or has an unsupported format, the program will crash with an error. For peace of mind, it is highly recommended to place your media in the same directory as the `Animura.exe` executable; this will prevent a multitude of errors.

> ### `vsync`

`vsync` — VSYNC operational parameters.

This parameter limits the frame rate relative to the monitor's refresh rate. Possible values range from `0` to `4`.

- `0` — no frame rate limitation via VSYNC.
- `1` — the frame rate will aim to match the monitor's refresh rate.
- `2` — the frame rate will aim to match 1/2 of the monitor's refresh rate.
- `3` — the frame rate will aim to match 1/3 of the monitor's refresh rate.
- `4` — the frame rate will aim to match 1/4 of the monitor's refresh rate.

The higher the value, the fewer frames per second you will get. Consequently, the lower the load on the machine.

Important: If `vsync` is greater than zero, the `latency` parameter must be set to `0`.

## System Load and Comfort

The program is designed to operate with the minimum possible load on the GPU, but the actual load depends on your video file, screen resolution, system performance, and configuration settings.

If **Animura** heavily loads your machine:

- Increase the `latency`.
- Set `vsync` to `2`, `3`, or `4`.
- Use a video with a lower resolution.

Important: As a reminder, if `vsync` is greater than zero, the `latency` parameter must be `0`. Conversely, if `latency` is greater than zero, the `vsync` parameter must be `0`.

## Program Termination

Currently, the program lacks a built-in feature to terminate itself automatically. To stop the execution, simply end the task via the Task Manager. 

## Feedback

### This project is created and maintained by a single developer. I would greatly appreciate knowing that people are using my application, and your feedback is highly valuable to me. 

### You can support me by subscribing to my Telegram channel: https://t.me/KernelaStudio.

### Direct contact: https://t.me/Mr_Zefy.

## Please ensure you read the license agreement.
