#include "lib.hpp"

#include <cstdlib>

struct Buffer {
    explicit Buffer(const char* data) : _data(data) {}
    ~Buffer() {
        free((void*)_data);
    }
    const char* _data;
};

struct StringArg {
    explicit StringArg(Buffer** dest) : _temp(nullptr), _dest(dest) {}

#ifdef __APPLE__
    ~StringArg() {
        if (_temp == nullptr) {
            *_dest = nullptr;
        } else {
            CFIndex length = CFStringGetLength(_temp);
            CFIndex maxSize = CFStringGetMaximumSizeForEncoding(length, kCFStringEncodingUTF8) + 1;
            char* data = (char*)malloc(maxSize);
            CFStringGetCString(_temp, data, maxSize, kCFStringEncodingUTF8);
            *_dest = new Buffer(data);
            CFRelease(_temp);
        }
    }

    operator CFStringRef*() {
        return &_temp;
    }

    CFStringRef _temp;
#else
    ~StringArg() {
        *_dest = new Buffer(_temp);
    }

    operator const char**() {
        return &_temp;
    }

    const char* _temp;
#endif

    Buffer** _dest;
};

extern "C" {

IDeckLinkIterator* create_decklink_iterator_instance() {
    return CreateDeckLinkIteratorInstance();
}

HRESULT decklink_iterator_next(IDeckLinkIterator* iterator, IDeckLink** deckLinkInstance) {
    return iterator->Next(deckLinkInstance);
}

ULONG decklink_iterator_release(IDeckLinkIterator* obj) {
    return obj->Release();
}

HRESULT decklink_get_model_name(IDeckLink* decklink, Buffer** str) {
    return decklink->GetModelName(StringArg(str));
}

HRESULT decklink_query_interface(IDeckLink* decklink, REFIID iid, LPVOID* iface) {
    return decklink->QueryInterface(iid, iface);
}

ULONG decklink_release(IDeckLink* obj) {
    return obj->Release();
}

HRESULT decklink_attributes_get_flag(IDeckLinkAttributes* attr, BMDDeckLinkAttributeID cfgID, bool* value) {
    return attr->GetFlag(cfgID, value);
}

HRESULT decklink_attributes_get_int(IDeckLinkAttributes* attr, BMDDeckLinkAttributeID cfgID, int64_t* value) {
    return attr->GetInt(cfgID, value);
}

HRESULT decklink_attributes_get_float(IDeckLinkAttributes* attr, BMDDeckLinkAttributeID cfgID, double* value) {
    return attr->GetFloat(cfgID, value);
}

HRESULT decklink_attributes_get_string(IDeckLinkAttributes* attr, BMDDeckLinkAttributeID cfgID, Buffer** value) {
    return attr->GetString(cfgID, StringArg(value));
}

ULONG decklink_attributes_release(IDeckLinkAttributes* attr) {
    return attr->Release();
}

HRESULT decklink_status_get_flag(IDeckLinkStatus* status, BMDDeckLinkStatusID statusID, bool* value) {
    return status->GetFlag(statusID, value);
}

HRESULT decklink_status_get_int(IDeckLinkStatus* status, BMDDeckLinkStatusID statusID, int64_t* value) {
    return status->GetInt(statusID, value);
}

ULONG decklink_status_release(IDeckLinkStatus* status) {
    return status->Release();
}

HRESULT decklink_input_get_display_mode_iterator(IDeckLinkInput* input, IDeckLinkDisplayModeIterator** iterator) {
	return input->GetDisplayModeIterator(iterator);
}

ULONG decklink_input_release(IDeckLinkInput* input) {
	return input->Release();
}

HRESULT decklink_output_get_display_mode_iterator(IDeckLinkOutput* output, IDeckLinkDisplayModeIterator** iterator) {
	return output->GetDisplayModeIterator(iterator);
}

ULONG decklink_output_release(IDeckLinkOutput* output) {
	return output->Release();
}

HRESULT decklink_display_mode_iterator_next(IDeckLinkDisplayModeIterator* iterator, IDeckLinkDisplayMode** deckLinkDisplayMode) {
	return iterator->Next(deckLinkDisplayMode);
}

ULONG decklink_display_mode_iterator_release(IDeckLinkDisplayModeIterator* iterator) {
	return iterator->Release();
}

BMDDisplayMode decklink_display_mode_get_display_mode(IDeckLinkDisplayMode* mode) {
	return mode->GetDisplayMode();
}

HRESULT decklink_display_mode_get_name(IDeckLinkDisplayMode* mode, Buffer** value) {
	return mode->GetName(StringArg(value));
}

ULONG decklink_display_mode_release(IDeckLinkDisplayMode* mode) {
	return mode->Release();
}

const void* buffer_data(Buffer* buf) {
    return buf->_data;
}

void buffer_release(Buffer* obj) {
    if (obj != nullptr) {
        delete obj;
    }
}

}
