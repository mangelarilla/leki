LEKI es un bot que permite crear y editar eventos de forma eficiente, así como limpiar el canal una vez finalizado, sin tener que depender de un administrador. En este tutorial, explicaremos las funcionalidades principales de la herramienta mediante un ejemplo práctico dividido en varios pasos.

## PASO 1: Llamar al bot para crear el evento

Nos situaremos en el canal donde queramos crear el evento (*domingo-b*, por ejemplo) y escribiremos en el chat el comando `/events`; en este momento Discord reconocerá dicho comando y solo tendremos que ejecutarlo pulsando **ENTER**, tal y como podemos ver en la siguiente captura:

![image](https://github.com/mangelarilla/leki/assets/4896595/8c044ffd-83e1-49ab-9b35-567c7f412c33)

## PASO 2: Seleccionar tipo de evento

En este momento, LEKI nos pregunta el tipo de evento que queremos realizar; existen tres categorías: Trial, PvP o Genérico, y, obviamente, deberemos elegir una; en nuestro ejemplo vamos a seleccionar la primera opción (Trial):

![image](https://github.com/mangelarilla/leki/assets/4896595/a6a2574e-3dcb-4277-b101-e7d28d03821a)

## PASO 3: Introducción de datos básicos del evento

En este punto debemos introducir los datos genéricos del evento; los hay obligatorios (Llevan el símbolo “*” indicado, y hacen referencia al título del evento, duración y descripción), y otros que se pueden rellenar adicionalmente, como por ejemplo addons que el RL pueda considerar necesarios para poder realizar la trial o guías de consulta de la misma; en cualquier caso, al finalizar debemos pulsar el botón **ENVIAR**, y en el supuesto de que hubiese algún dato que no se haya rellenado correctamente, nos aparecerá un mensaje arriba.

![image](https://github.com/mangelarilla/leki/assets/4896595/8a107d54-0fd1-4134-9f60-12022293746b)

## PASO 4: Previsualización del evento

Continuamos rellenando datos del evento; en este caso indicaremos tanto la composición del mismo (En este caso, como nos referimos a una trial, nos solicitará los roles que incluiremos en la misma, sugiriendo una estructuración por defecto, aunque también se puede cambiar pulsando el botón **MODIFICAR**); al terminar continuaremos pulsando el botón **CONFIRMAR**:

![image](https://github.com/mangelarilla/leki/assets/4896595/f4775200-5f51-4d11-a812-498bd309f66e)

## PASO 5: Tipo de roster

Debemos el tipo de roster de evento, indicando si es abierto, semiabierto o cerrado (Esto nos indica si los participantes se pueden apuntar libremente o no).
En nuestro caso pulsamos el botón **ABIERTO** para proceder al siguiente punto.

![image](https://github.com/mangelarilla/leki/assets/4896595/f876048f-17c8-4ed9-9a21-f7c209e719ae)

## PASO 6: Selección de canal, fecha y hora

Ahora debemos elegir, el canal donde realizaremos en nuestro caso la trial, así como fecha y hora del evento.
Para nuestro ejemplo elegiremos el domingo 28 a las 22:00 horas, por tanto, **teclearemos** el canal elegido, y posteriormente seleccionaremos fecha y hora, tal y como se ve en las dos capturas siguientes:

![image](https://github.com/mangelarilla/leki/assets/4896595/d61787a5-c5fc-4819-92a6-34d874bc435e)

![image](https://github.com/mangelarilla/leki/assets/4896595/6405c369-b66c-4474-91f9-0ada8b6dc539)

Aquí podemos ver como se ha creado ya el evento con las especificaciones anteriores, y en la fecha indicada:

![image](https://github.com/mangelarilla/leki/assets/4896595/cff162d2-9c81-42d1-84a9-730e1f7a5d3a)

Asimismo, al crear un evento, ya queda anotado en el calendario de eventos de la guild, con lo que no es necesario crearlo, tal y como había que hacer antes.

![image](https://github.com/mangelarilla/leki/assets/4896595/e19dc77c-ec94-4efb-9f69-0158345d3769)

A partir de este punto, los participantes en el evento ya se pueden apuntar directamente; simplemente tienen que **pulsar en el rol que desean ocupar** y ya aparecen dentro del mismo, sin necesidad de poner un + en el canal como hasta ahora, y por tanto ahorrando mucho trabajo de gestión al RL; también podrán apuntarse como **reservas o con flexibilidad de roles**, e incluso **desapuntarse** en el caso de que al final no puedan asistir.

![image](https://github.com/mangelarilla/leki/assets/4896595/eb999dfa-7206-4b16-92ab-b82e4a90e14e)

En este momento el evento ya está definido y anotado en el canal, pero es importante notificarlo de forma masiva con un `@everyone`; esto se hace de la manera habitual en un mensaje de texto, ya sea dentro del canal del evento, o bien como un mensaje general con un enlace al mismo. Igualmente, **también se debe seguir creando la entrada correspondiente en el calendario Excel de eventos**, ya que el bot no puede acceder a él.

## PASO 7: Edición y borrado del evento

Los eventos pueden ser editados o eliminados directamente desde la propia ventana del canal; para ello pulsaremos en los tres puntos del mensaje del bot que hemos publicado y accederemos, dentro del menú que nos sale, a la opción **APLICACIONES**; dentro de dicha opción podremos seleccionar si queremos editar el evento (Teniendo en cuenta que dicha edición se realiza en modo texto), o bien eliminar el mismo; esto es importante porque no solo borra el mensaje del evento, sino que limpia todo el canal, de forma que cada RL se puede encargar de administrar sus propios eventos sin depender de un administrador de Discord.

![image](https://github.com/mangelarilla/leki/assets/4896595/af78a91a-aeb5-47e7-8257-3f7e190ecbaa)

