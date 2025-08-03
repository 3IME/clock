import init, { start_clock } from "./pkg/thoo_clock.js";

let alarmTime = null;
let alarmActive = false;
let alarmInterval = null;
let sound = null;

async function run() {
  await init();
  start_clock();

  const input = document.getElementById('altime');
  const setBtn = document.getElementById('set');
  const alarmDiv = document.getElementById('alarm1');
  const alarmOffBtn = document.getElementById('turnOffAlarm');

  sound = new Audio('alarm.mp3'); 
  sound.loop = true;

  const originalBg = getComputedStyle(document.body).backgroundColor;

  function startBodyBlink() {
    let on = false;
    alarmInterval = setInterval(() => {
      document.body.style.backgroundColor = on ? 'red' : originalBg;
      on = !on;
    }, 500);
  }

  function stopBodyBlink() {
    clearInterval(alarmInterval);
    document.body.style.backgroundColor = originalBg;
  }

  function checkAlarm() {
    if (!alarmTime || alarmActive) return;  // Ne rien faire si alarme active ou pas d'heure

    const now = new Date();
    let nowHours = now.getHours();
    if (nowHours === 0) nowHours = 12;
    else if (nowHours > 12) nowHours -= 12;

    const nowMinutes = now.getMinutes();

    const nowTotalMinutes = nowHours * 60 + nowMinutes;
    const alarmTotalMinutes = alarmTime.hours * 60 + alarmTime.minutes;

    if (nowTotalMinutes >= alarmTotalMinutes) {
      alarmActive = true;
      alarmDiv.style.display = 'block';
      sound.play();
      startBodyBlink();
      console.log('Alarme déclenchée !');
    }
  }

  function scheduleAlarmCheck() {
    const now = new Date();
    const delay = 1000 - now.getMilliseconds();
    setTimeout(() => {
      checkAlarm();
      scheduleAlarmCheck();
    }, delay);
  }

  setBtn.addEventListener('click', (e) => {
    e.preventDefault();

    const val = input.value.trim();
    const regex = /^(\d{1,2}):(\d{2})$/;
    const match = val.match(regex);

    if (!match) {
      alert('Format heure invalide, utilisez hh:mm (0-12 heures)');
      return;
    }

    let h = parseInt(match[1], 10);
    let m = parseInt(match[2], 10);

    if (h < 0 || h > 12 || m < 0 || m > 59) {
      alert('Heure doit être entre 0 et 12, minutes entre 0 et 59');
      return;
    }

    alarmTime = { hours: h, minutes: m };
    alarmActive = false;
    alarmDiv.style.display = 'none';
    stopBodyBlink();
    sound.pause();
    sound.currentTime = 0;
	
	alert(`Alarme réglée à ${h.toString().padStart(2, '0')}:${m.toString().padStart(2, '0')}`);
  

  });

  alarmOffBtn.addEventListener('click', (e) => {
    e.preventDefault();

    alarmActive = false;
    alarmTime = null;    // <-- Ajouté : désactive complètement l'alarme programmée
    alarmDiv.style.display = 'none';
    stopBodyBlink();
    sound.pause();
    sound.currentTime = 0;

  });

  scheduleAlarmCheck();
}

run();
