// Slide navigation, deep links and copy buttons. No dependencies.
(function () {
  'use strict';

  var slides = Array.prototype.slice.call(document.querySelectorAll('.slide'));
  var counterCurrent = document.getElementById('counter-current');
  var counterTotal = document.getElementById('counter-total');
  var desktop = window.matchMedia('(min-width: 1024px)');
  var reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)');
  var current = 0;

  if (counterTotal) counterTotal.textContent = String(slides.length);

  function indexFromHash() {
    var id = decodeURIComponent(location.hash.slice(1));
    for (var i = 0; i < slides.length; i++) {
      if (slides[i].id === id) return i;
    }
    return -1;
  }

  function setCurrent(index) {
    current = index;
    if (counterCurrent) counterCurrent.textContent = String(index + 1);
    var hash = '#' + slides[index].id;
    if (location.hash !== hash && window.history && history.replaceState) {
      history.replaceState(null, '', hash);
    }
  }

  function goTo(index) {
    index = Math.max(0, Math.min(slides.length - 1, index));
    slides[index].scrollIntoView({ behavior: reducedMotion.matches ? 'auto' : 'smooth', block: 'start' });
    setCurrent(index);
  }

  // Reveal slides as they enter the viewport.
  if ('IntersectionObserver' in window) {
    var observer = new IntersectionObserver(function (entries) {
      entries.forEach(function (entry) {
        if (entry.isIntersecting) {
          entry.target.classList.add('is-visible');
          observer.unobserve(entry.target);
        }
      });
    }, { threshold: 0.08 });
    slides.forEach(function (slide) { observer.observe(slide); });
  } else {
    slides.forEach(function (slide) { slide.classList.add('is-visible'); });
  }

  // The current slide is the last one whose top crossed the upper third of the viewport.
  // Works even when a slide is taller than the screen (common on phones).
  function syncFromScroll() {
    var line = window.innerHeight / 3;
    var index = 0;
    for (var i = 0; i < slides.length; i++) {
      if (slides[i].getBoundingClientRect().top <= line) index = i;
    }
    if (index !== current || location.hash !== '#' + slides[index].id) setCurrent(index);
  }

  var ticking = false;
  window.addEventListener('scroll', function () {
    if (ticking) return;
    ticking = true;
    window.requestAnimationFrame(function () { ticking = false; syncFromScroll(); });
  }, { passive: true });

  var initial = indexFromHash();
  if (initial > 0) {
    slides[initial].scrollIntoView({ block: 'start' });
    current = initial;
    if (counterCurrent) counterCurrent.textContent = String(initial + 1);
  }

  window.addEventListener('hashchange', function () {
    var index = indexFromHash();
    if (index >= 0) goTo(index);
  });

  function toggleFullscreen() {
    var doc = document;
    if (!doc.fullscreenElement) {
      if (doc.documentElement.requestFullscreen) doc.documentElement.requestFullscreen().catch(function () {});
    } else if (doc.exitFullscreen) {
      doc.exitFullscreen();
    }
  }

  document.addEventListener('keydown', function (event) {
    if (!desktop.matches) return;
    if (event.altKey || event.ctrlKey || event.metaKey) return;
    var target = event.target;
    var tag = target && target.tagName;
    if (tag === 'INPUT' || tag === 'TEXTAREA' || (target && target.isContentEditable)) return;
    var onControl = tag === 'BUTTON' || tag === 'A';

    switch (event.key) {
      case 'ArrowRight':
      case 'ArrowDown':
      case 'PageDown':
        goTo(current + 1); break;
      case ' ':
      case 'Spacebar':
        if (onControl) return;
        if (event.shiftKey) goTo(current - 1); else goTo(current + 1);
        break;
      case 'ArrowLeft':
      case 'ArrowUp':
      case 'PageUp':
        goTo(current - 1); break;
      case 'Home':
        goTo(0); break;
      case 'End':
        goTo(slides.length - 1); break;
      case 'f':
      case 'F':
        toggleFullscreen(); break;
      default:
        return;
    }
    event.preventDefault();
  });

  // Copy buttons: copy exactly the text shown in the block.
  function fallbackCopy(text) {
    var area = document.createElement('textarea');
    area.value = text;
    area.setAttribute('readonly', '');
    area.style.position = 'fixed';
    area.style.top = '-1000px';
    area.style.opacity = '0';
    document.body.appendChild(area);
    area.select();
    var ok = false;
    try { ok = document.execCommand('copy'); } catch (e) { ok = false; }
    document.body.removeChild(area);
    return ok ? Promise.resolve() : Promise.reject(new Error('copy failed'));
  }

  function copyText(text) {
    if (navigator.clipboard && window.isSecureContext) {
      return navigator.clipboard.writeText(text).catch(function () { return fallbackCopy(text); });
    }
    return fallbackCopy(text);
  }

  Array.prototype.forEach.call(document.querySelectorAll('.copy'), function (button) {
    var block = button.closest('.block');
    var code = block && block.querySelector('pre code');
    if (!code) return;
    var label = button.textContent;
    var timer;
    button.addEventListener('click', function () {
      copyText(code.textContent).then(function () {
        button.textContent = '¡Copiado!';
        button.classList.add('is-copied');
      }, function () {
        button.textContent = 'Selecciona y copia';
      }).then(function () {
        clearTimeout(timer);
        timer = setTimeout(function () {
          button.textContent = label;
          button.classList.remove('is-copied');
        }, 1800);
      });
    });
  });
})();
