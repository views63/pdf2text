# pdf2text
为解决使用 `lwarp `  过程中内部调用 `poppler` 中的 `pdftotext.exe` 转换 `pdf` 时出现的文章顺序错误（https://github.com/CTeX-org/forum/issues/18 ）

用法：

编译后把 `pdf2text.exe` 复制到 `pdftotext.exe` 同一目录，找到 `lwarpmk.lua` 中的 `pdftohtml` 函数，把

```lua
os.execute("pdftotext  -enc " .. pdftotextenc .. "  -nopgbrk  -layout "
    .. sourcename .. "_html.pdf " .. sourcename .. "_html.html")
```

改为 

```lua
os.execute("pdf2text " .. sourcename .. "_html.pdf " .. sourcename .. "_html.html")
```

