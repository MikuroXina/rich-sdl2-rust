#define SDL_MAIN_HANDLED
#include <SDL2/SDL.h>
#include <SDL2/SDL_syswm.h>
#include <SDL2/SDL_vulkan.h>

#ifdef RICH_SDL2_RUST_TTF
#include <SDL2/SDL_ttf.h>
#endif

#ifdef RICH_SDL2_RUST_MIXER
#include <SDL2/SDL_mixer.h>
#endif

#ifdef RICH_SDL2_RUST_IMAGE
#include <SDL2/SDL_image.h>
#endif

#ifdef RICH_SDL2_RUST_NET
#include <SDL2/SDL_net.h>
#endif
