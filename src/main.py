import json
import ctypes


def get_urls():
    """ impl your get urls func """
    urls = [
        b"http://example.com/",
        b"https://www.yahoo.com/",
    ]
    return urls


def get_htmls_as_dict_from(urls):
    """ get html content using rust's dylib concurrently """
    DYLIB_PATH = "target/release/libhyper_thread.dylib"
    lib = ctypes.cdll.LoadLibrary(DYLIB_PATH)
    C_CHAR_P_P = ctypes.c_char_p * len(urls)
    c_urls = C_CHAR_P_P(*urls)
    lib.get_htmls_from.argtypes = (C_CHAR_P_P, ctypes.c_size_t)
    lib.get_htmls_from.restype = ctypes.c_void_p
    htmls = lib.get_htmls_from(c_urls, len(urls))
    try:
        return json.loads(ctypes.cast(htmls, ctypes.c_char_p).value.decode('utf-8'))
    except:
        return None

if __name__ == '__main__':
    urls = get_urls()
    url2html = get_htmls_as_dict_from(urls)
    print(url2html.keys())
