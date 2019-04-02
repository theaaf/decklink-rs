#include <DeckLinkAPI.h>

struct Buffer;

extern "C" {

IDeckLinkIterator* create_decklink_iterator_instance();
HRESULT decklink_iterator_next(IDeckLinkIterator* iterator, IDeckLink** deckLinkInstance);
ULONG decklink_iterator_release(IDeckLinkIterator* obj);

HRESULT decklink_get_model_name(IDeckLink* decklink, Buffer** str);
HRESULT decklink_query_interface(IDeckLink* decklink, REFIID iid, LPVOID* iface);
ULONG decklink_release(IDeckLink* obj);

HRESULT decklink_attributes_get_flag(IDeckLinkAttributes* attr, BMDDeckLinkAttributeID cfgID, bool* value);
HRESULT decklink_attributes_get_int(IDeckLinkAttributes* attr, BMDDeckLinkAttributeID cfgID, int64_t* value);
HRESULT decklink_attributes_get_float(IDeckLinkAttributes* attr, BMDDeckLinkAttributeID cfgID, double* value);
HRESULT decklink_attributes_get_string(IDeckLinkAttributes* attr, BMDDeckLinkAttributeID cfgID, Buffer** value);
ULONG decklink_attributes_release(IDeckLinkAttributes* attr);

HRESULT decklink_status_get_flag(IDeckLinkStatus* status, BMDDeckLinkStatusID statusID, bool* value);
HRESULT decklink_status_get_int(IDeckLinkStatus* status, BMDDeckLinkStatusID statusID, int64_t* value);
ULONG decklink_status_release(IDeckLinkStatus* status);

HRESULT decklink_input_get_display_mode_iterator(IDeckLinkInput* input, IDeckLinkDisplayModeIterator** iterator);
ULONG decklink_input_release(IDeckLinkInput* input);

HRESULT decklink_output_get_display_mode_iterator(IDeckLinkOutput* output, IDeckLinkDisplayModeIterator** iterator);
ULONG decklink_output_release(IDeckLinkOutput* output);

HRESULT decklink_display_mode_iterator_next(IDeckLinkDisplayModeIterator* iterator, IDeckLinkDisplayMode** deckLinkDisplayMode);
ULONG decklink_display_mode_iterator_release(IDeckLinkDisplayModeIterator* iterator);

BMDDisplayMode decklink_display_mode_get_display_mode(IDeckLinkDisplayMode* mode);
HRESULT decklink_display_mode_get_name(IDeckLinkDisplayMode* mode, Buffer** value);
ULONG decklink_display_mode_release(IDeckLinkDisplayMode* mode);

const void* buffer_data(Buffer* str);
void buffer_release(Buffer* str);

}
