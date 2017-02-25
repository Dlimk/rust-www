---
layout: hu-HU/default
title: A Rust csapat &middot; A Rust programozási nyelv
---

<style type="text/css">
.headshot {
  border: 1px solid #888;
  width: 140px;
}

.person {
  display: inline-block;
  position: relative;
  margin-bottom: 20px;
}
.lead { font-weight: bold; }
.lead .name::after { content: " (lead)"; }
.details {
  display: none;
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  background: rgba(0, 0, 0, 0.5);
  color: white;
  font-weight: normal;
}
.person:hover .details {
   display: block;
}

.headshots {
  text-align: center;
  margin: 0px auto;
  padding: 0;
  width: 700px;
  max-width: 100%;
  list-style: none;
}
</style>

# A Rust csapat

A Rust projektet néhány csapat
[vezeti](https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md)
, melyek mindegyike egy adott területtel foglalkozik. Lent látható a névsor,
ábécésorrendben.

Hogy kapcsolatba lépj egy csapattal, tedd fel a kérdéset, vagy írj megjegyzést
[az Internals fórumon](https://internals.rust-lang.org/), és címkézd meg a hozzászólást
a megfelelő csapat nevével. Vedd figyelembe, hogy a biztonsági megfontolásokkal
kapcsolatos közzétételeknek követniük kell a
[Rust biztonsági közzétételi folyamatot](security.html). 

{% for team in site.data.team.teams %}
<section id="{{ team.name | replace:' ','-' }}">
<h2> {{ page.localized-teams[team.name].name | default: team.name }} </h2>

<strong>Felelősség</strong>: <em>{{ page.localized-teams[team.name].responsibility | default: team.responsibility }}</em>

<br />

{% if team.email %}
  <strong>Kapcsolat</strong>:
  <a href="mailto:{{ team.email | uri_escape }}">{{ team.email }}</a>
{% endif %}

<ul class="headshots">
{% for nick in team.members %}
  {% assign person = site.data.team.people[nick] %}
  {% if person.site %}
    {% assign sitename = person.site %}
  {% else %}
    {% assign sitename = "github" %}
  {% endif %}
  {% assign website = site.data.team.sites[sitename] %}
  <li class="person {% if team.lead and team.lead == nick %}lead{% endif %}">
  <a href="{{ website.url | replace:'%nick',nick }}">
    <div class="name">{{ person.name }}</div>
    <div class="details">
      <div>irc: {% if person.irc %}{{ person.irc }}{% else %}{{ nick }}{% endif %}</div>
      {% if person.ex-teams %}
      <div>teams: {% for ex-team in person.ex-teams %}{% if forloop.first == false %}, {% endif %}{{ page.localized-ex-teams[ex-team] | default: ex-team }}{% endfor %}</div>
      {% endif %}
    </div>
    <img class="headshot" src="{{ website.avatar | replace:'%nick',nick }}" alt="{{ person.name }}">
  </a>
</li>
{% endfor %}
</ul>
</section>
{% endfor %}
