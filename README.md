# Slideshow

A web slideshow app for a very specific task. Not intended for public use.

`~/.config/openbox/autostart`:
```
xset s off
xset -dpms
xset s noblank

for output in HDMI-1 HDMI-2; do
  if xrandr --query | grep -q "^$output connected"; then
    xrandr --output "$output" --rotate left
  fi
done

unclutter -idle 0.5 -root &


chromium --kiosk --noerrdialogs --disable-infobars \
  --disable-session-crashed-bubble --no-first-run --incognito \
  --overscroll-history-navigation=0 \
  --disable-features=Translate,TranslateUI \
  --disable-accelerated-video-decode \
  --remote-debugging-port=9222 \
  https://zviracek.github.io/Slideshow/ &
```
