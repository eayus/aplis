#include <efi/boot-services.h>
#include <efi/runtime-services.h>
#include <efi/system-table.h>
#include <efi/types.h>
 
#include <stdbool.h>
#include "print.c"


// Number of 4KiB pages
#define HEAP_SIZE 10

 
/* I'm too lazy to type this out five times */
#define ERR(x) if(EFI_ERROR((x))) return (x)
 
efi_status efi_main(efi_handle handle __attribute__((unused)), efi_system_table *st) {
	efi_status status;
	efi_input_key key;
 
	/* reset the watchdog timer */
	st->BootServices->SetWatchdogTimer(0, 0, 0, NULL);
	ERR(status);
 
	/* clear the screen */
	status = st->ConOut->ClearScreen(st->ConOut);
	ERR(status);

	
	efi_physical_addr heap;
	status = st->BootServices->AllocatePages(AllocateAnyPages, EfiLoaderData, HEAP_SIZE, &heap);
	ERR(status);

	print(st->ConOut, L"Yay!");


	/* flush console input buffer */
	status = st->ConIn->Reset(st->ConIn, false);
	ERR(status);
 
	/* poll for a keystroke */
	while((status = st->ConIn->ReadKeyStroke(st->ConIn, &key)) == EFI_NOT_READY);
	ERR(status);
 
	return EFI_SUCCESS;
}
